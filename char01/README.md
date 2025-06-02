# Rust Echo Driver for Linux Kernel

A simple character device driver written in Rust that echoes back any data written to it.

## Why `cargo build/run` Doesn't Work

🚫 **This is not a regular Rust application** - it's a Linux kernel module with special requirements:

1. **Different Build System**:  
   Kernel modules must be compiled with the Linux kernel build system (kbuild), not Cargo.

2. **No Standard Library**:  
   Kernel code can't use Rust's `std` - it uses `core` and kernel-specific crates.

3. **Special Dependencies**:  
   Depends on Linux kernel internal APIs, not crates.io packages.

4. **Different Linking**:  
   Produces a `.ko` (kernel object) file, not a regular binary.

## Prerequisites

- Linux kernel v6.1+ with Rust support enabled
- Rust nightly toolchain
- `rust-for-linux` setup
- Kernel headers and build tools

## Building the Driver

1. **Place the code** in the kernel tree:  
   Save as `rust/echo_driver.rs`

2. **Add to build system**:  
   Edit `rust/Makefile` to include:
   ```makefile
   obj-$(CONFIG_SAMPLE_RUST) += echo_driver.o
