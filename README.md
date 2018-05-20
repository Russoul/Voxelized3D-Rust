# Voxelized3D-Rust

![UMDC + signed field geometry](uniform_manifold_dual_contouring.png)

![UMDC + perlin noise](umdc_perlin_noise.png)

![blocky terrain + perlin noise](cubic_terrain.png)

# Dependencies
* rust
* glfw3
* OpenGL >= 3.3.0
* libgc

# how to build on Ubuntu 16.04 - 17.10 (including installation of dependencies)
```
$ sudo apt-get update
$ sudo apt-get install curl
$ sudo apt-get install libglfw3-dev
$ sudo apt-get install libgc-dev
$ sudo apt-get install git
$ curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --channel=stable

$ cd <Directory where you want to compile Voxelized3D-Rust>
$ git clone https://github.com/Russoul/Voxelized3D-Rust
$ cd Voxelized3D-Rust
$ ~/.cargo/bin/cargo run --release
```
