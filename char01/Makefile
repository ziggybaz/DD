# Linux Kernel Rust Echo Driver Makefile

# Kernel build system integration
ifneq ($(KERNELRELEASE),)
    # Kernel build system is calling us (second pass)

    # Module name (must match your .rs filename without extension)
    obj-m := echo_driver.o

    # Additional Rust flags
    RUSTFLAGS_echo_driver.o := --extern kernel=$(src)/bindings_generated.rs

else
    # Normal make invocation (first pass)

    # Path to kernel build directory
    KERNELDIR ?= /lib/modules/$(shell uname -r)/build
    # For custom kernel: KERNELDIR ?= /path/to/your/kernel/source

    # Rust toolchain configuration
    export RUSTC=$(shell which rustc)
    export RUSTDOC=$(shell which rustdoc)
    export RUSTFMT=$(shell which rustfmt)
    export BINDGEN=$(shell which bindgen)

    # LLVM=1 is required for Rust support
    export LLVM=1

    .PHONY: all clean load unload test

    all:
        $(MAKE) -C $(KERNELDIR) M=$(PWD) modules

    clean:
        $(MAKE) -C $(KERNELDIR) M=$(PWD) clean
        rm -f *.o *.ko *.mod.* .*.cmd Module.symvers modules.order
        rm -f rust/*.o rust/*.ko rust/.*.cmd

    load: all
        sudo insmod echo_driver.ko || true
        @echo "Module loaded. Check dmesg for major number."
        @echo "Then run: sudo mknod /dev/echo c <major> 0 && sudo chmod 666 /dev/echo"

    unload:
        sudo rmmod echo_driver 2>/dev/null || true
        sudo rm -f /dev/echo

    test: load
        @echo "Running basic tests..."
        @echo "Test123" | sudo tee /dev/echo >/dev/null
        @echo "Contents of /dev/echo:"
        @cat /dev/echo
        @sudo rmmod echo_driver

    debug:
        $(MAKE) -C $(KERNELDIR) M=$(PWD) CONFIG_DEBUG_INFO=y modules

    clippy:
        cargo clippy --target=x86_64-unknown-none --no-default-features --features "core compiler_builtins" -- -D warnings

    help:
        @echo "Available targets:"
        @echo "  all       - Build the kernel module (default)"
        @echo "  clean     - Remove build artifacts"
        @echo "  load      - Build and load module"
        @echo "  unload    - Unload module and cleanup"
        @echo "  test      - Run basic functionality test"
        @echo "  debug     - Build with debug symbols"
        @echo "  clippy    - Run Clippy linter"
        @echo "  help      - Show this help message"

endif
