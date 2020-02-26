# Unimplemented OS

A naive os written in pure Rust.

Maybe called Untitled OS or Ugly OS after implemention.

## How to Run?

You need a nightly Rust compiler and have `cargo-xbuild` and `bootimage` installed

```
cargo install cargo-xbuild bootimage
```

Then you can build the project by running:

```
cargo xbuild
```

To create a bootable disk image, run:

```
cargo bootimage
```

Then you can run the disk image in QEMU through:

```
cargo xrun
```

## How to test?

To run the unit and integration tests, execute: 

```
cargo xtest
```

# Roadmap

- [x] Rust build-in test framwork
- [x] Trap and Interrupt
- [x] Virtual Memory and Management
- [ ]  process and Scheduling 
- [ ]  file system
- [ ]  shell
- [ ]  advanced method

# ref

- cs140e:standford  (bot 2018 and 2020 edition) https://cs140e.sergio.bz
- Tock OS:stanford
- XV6:standford
- rCore https://github.com/rcore-os/rCore
- Blog_OS https://github.com/phil-opp/blog_os
- "30天自制操作系统" [日] 川合秀实


