# My Rust Character Device Implementation

I'm usually on and off with these things, I even wrote a dummy driver a while back @ https://github.com/ziggybaz/noop-driver

Anyway, this is the simplest Character device or 'char' (pronounce it however you want bro, no worries) device I could come up with.

a char device is one that can be accessed as a stream of bytes and in it's most basic form implements (open, read, write and close) system calls e.g

```rust
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

    writer.write_slice(&buffer_guard[position..position * to_copy])?;

    pr_info!("read {} bytes from echo device\n", to_copy);
    Ok(to_copy)
}
```

it doesn't do much but by the end of time I'll probably have gotten around to building better and more efficient ones, there are alot of things missing for example the security policy here is wanting, the only thing I safeguard against here is buffer overflow checks, you can just imagine that's why I chose Rust over C, hehe e.g:

```rust
if *length_guard >= ECHO_BUF_SIZE {
    return Err(Error::ENOSPC)
}
```
