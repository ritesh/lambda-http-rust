# HTTP function lambda

This project is here to illustrate how to use rust to write AWS Lambda functions. Most of the info here is distilled [from here](https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda/). **The code in the blog post will not work with recent versions of the lambda-runtime (~v0.5 and up), so use the code here as a reference**.  There's a bit of ceremony involved in getting things going. I'm using the Windows Subsystem for Linux (WSL) via Visual Studio Code's awesome remote containers feature, as I'm not all good with Powershell and its ilk. 

## Pre-reqs
  * WSL v1 with Ubuntu-20.04. I cannot use WSLv2 as I'm running this in Amazon Workspaces, which is already a virtualised environment
  * Rust installed in WSL (see the rust website on how to do this)
  * Install `musl-gcc` to enable cross-compilation for the `x86_64-linux-musl-gcc` target. This is `sudo apt install -y musl-tools`. I'm not 100% sure if this is required (since we want to target Linux anyway), but I know that `musl` provides an alternative to `libc` which varies wildly between different Linuxes. Doing this also helps if you want to build on macOS too.
  * Create a new project `cargo new mycoolproject --bin` with a `Cargo.toml` that looks like what I have here. 
  * Also in your project root `mkdir .cargo`, and create a file called `config` that looks like this:
  ```
  [target.x86_64-unknown-linux-musl]
  linker = "x86_64-linux-musl-gcc"
  ```
  This tells cargo how to link your code for the `x86_64-linux-musl-gcc` target. On WSL, this binary doesn't exist so you have to do

  ```
  sudo ln -s /usr/bin/musl-gcc /usr/bin/x86_64-linux-musl-gcc
  ```
## Build
To build release targets (no debugging symbols)

```
cargo build --release --target x86_64-unknown-linux-musl
```

Usual builds

```
cargo build
```


That's it!

