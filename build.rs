extern crate cc;
extern crate cmake;

fn main() {
    cc::Build::new()
        .file("./src/C/util.c")
        .file("./src/C/glad.c")
        .file("./src/C/qef_solver.cpp")
        .include("./src/H")
        .include("include")
        .compile("rsutil");

    let dst = cmake::build("voxelized-3d-cuda");

    println!("cargo:rustc-link-search=native={}", "./voxelized-3d-cuda/cmake-build-release"); //TODO
    println!("cargo:rustc-link-lib=static=stdc++");
    println!("cargo:rustc-link-lib=static=voxelized3d");
    
    
}


