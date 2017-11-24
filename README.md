# Voxelized2D-Rust

# Dependencies
* rustc
* cargo(rust)
* libgc
* glfw3
* OpenGL >= 3.3.0

# installation on Ubuntu 16.04 - 17.10
```
$ sudo apt-get update
$ sudo apt-get install curl
$ sudo apt-get install libgc-dev
$ sudo apt-get install libglfw3-dev
$ sudo apt-get install git
$ curl https://sh.rustup.rs -sSf | sh

$ cd <Directory where you want to compile Voxelized2D-Rust>
$ git clone https://github.com/Russoul/Voxelized2D-Rust
$ cd Voxelized2D-Rust
$ ~/.cargo/bin/cargo run --release
```