# Voxelized3D-Rust

[![](https://github.com/Russoul/Voxelized3D-Rust/workflows/macOS/badge.svg)](https://github.com/Russoul/Voxelized3D-Rust/actions?query=workflow%3A"macOS")
[![](https://github.com/Russoul/Voxelized3D-Rust/workflows/Windows/badge.svg)](https://github.com/Russoul/Voxelized3D-Rust/actions?query=workflow%3A"Windows")
# Dependencies
* rust(cargo)
* OpenGL >= 3.3.0 (detected automatically)
* cmake
* blas, lapack (or accelerate framework on Mac, distributed with the OS)

# Platforms
* tested on mac
* should(probably) work on linux without code changes provided that all depencies above are found(there may be a problem with GL though ...)
* won't work on windows without code changes (this concerns matrix libraries and GL)

# Pics
![UMDC + signed field geometry](imgs/uniform_manifold_dual_contouring.png)

![UMDC + perlin noise](imgs/umdc_perlin_noise.png)

![blocky terrain + perlin noise](imgs/cubic_terrain.png)

![perlin noise minus sphere](imgs/umdc_noise_minus_sphere.png)


