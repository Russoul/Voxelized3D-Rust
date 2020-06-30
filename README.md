# Voxelized3D-Rust

[![](https://github.com/Russoul/Voxelized3D-Rust/workflows/macOS/badge.svg)](https://github.com/Russoul/Voxelized3D-Rust/actions?query=workflow%3A"macOS")
[![](https://github.com/Russoul/Voxelized3D-Rust/workflows/Windows/badge.svg)](https://github.com/Russoul/Voxelized3D-Rust/actions?query=workflow%3A"Windows")

# Dependencies
* rust(cargo)
* OpenGL >= 3.3.0 (detected automatically)
* cmake
* gfortran (no need for this on Mac)
* xorg-dev package for linux with X11 window system
* OpenBlas (or accelerate framework on Mac, distributed with the OS, no need to install anything)

# Pics
![UMDC + signed field geometry](imgs/uniform_manifold_dual_contouring.png)

![UMDC + perlin noise](imgs/umdc_perlin_noise.png)

![blocky terrain + perlin noise](imgs/cubic_terrain.png)

![perlin noise minus sphere](imgs/umdc_noise_minus_sphere.png)


