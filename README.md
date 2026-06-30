Following the tutorial on writing an OS in Rust

## Creating an image

### Setup

```sh
cargo install bootimage
rustup component add llvm-tools-preview
```

### Creation

```sh
cargo bootimage
```

### Run with QEMU

```sh
qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown_jernel/debug/bootimage-jernel.bin
```
