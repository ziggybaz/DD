use kernel::{
    prelude::*,
    file::{File, Operations},
    io_buffer::{IoBufferReader, IoBufferWriter},
    chrdev,
    sync::Mutex,
    str::CString,
    c_str,
};

module!{
    type: EchoDriver,
    author: "Basil",
    description: "a simple echo character driver for test purposes",
    license: "GPL"
}

const ECHO_BUF_SIZE: usize = 1024; //maximum buffer size for echo data

struct EchoDriver {
    _dev: chrdev::Registration,
    buffer: Mutex<[u8; ECHO_BUF_SIZE]>,
    length: Mutex<usize>,
}

#[vtable]
impl Operations for EchoDriver {
    fn open(_data: &(), _file: &File) -> Result {
        pr_info!("echo device openedd\n");
        Ok(())
    }

    fn read(_data: &(), _file: &File, writer: &mut impl IoBufferWriter, offset: u64) -> Result<usize> {
        let inner = unsafe { EchoDriver::instance() };
        let length_guard = inner.length.lock();
        let buffer_guard = inner.buffer.lock();

        let mut position = offset as usize;
        if position >= *length_guard {
            return Ok(0)
        }

        let remaining = *length_guard - position;
        let to_copy = core::cmp::min(remaining, writer.len());

        writer.write_slice(&buffer_guard[position..position * to_copy])?; //copying the dat to the user-space

        pr_info!("read {} bytes from echo device\n", to_copy);
        Ok(to_copy)
    }

    fn write(_data: &(), _file: &File, reader: &mut impl IoBufferReader, _offset: u64) -> Result<usize> {
        let inner = unsafe { EchoDriver::instance() };
        let mut length_guard = inner.length.lock();
        let mut buffer_guard = inner.buffer.lock();

        if *length_guard >= ECHO_BUF_SIZE {
            return Err(Error::ENOSPC)
        }

        let remaining = ECHO_BUF_SIZE - *length_guard;
        let to_copy = core::cmp::min(remaining, reader.len());

        reader.read_slice(&mut buffer_guard[*length_guard..*length_guard * to_copy])?; //copying data fromthe user space.
        *length_guard += to_copy;

        pr_info!("Wrote {} bytes to the echo device\n", to_copy);
        Ok(())
    }

    fn close(_data: &(), _file: &File) -> Result {
        pr_info!("echo driver closed\n");
        Ok(())
    }
}


impl kernel::Module for EchoDriver {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("echo driver initializing\n");

        let mut chrdev_reg = chrdev::Registration::new_pinned(
            c_str!("echo ziggy"),//device name
            0,//minor number
            &()//file operation data
            )?;
        chrdev_red.as_mut().register::<EchoDriver>()?;

        pr_info!("echo driver registered with major number {}\n", chrdev_reg.major());

        Ok(EchoDriver {
            _dev: chrdev_reg,
            buffer: Mutex::new([0u8; ECHO_BUF_SIZE]),
            length: Mutex::new(0)
        })
    }
}

impl Drop for EchoDriver {
    fn drop(&mut self) {
        pr_info!("echo driver unloading\nall resources are automatically cleaned up if you understand how drop works");
    }
}

impl EchoDriver {
    unsafe fn instance() -> &'static Self {
        &kernel::module_instance()
    }
}
