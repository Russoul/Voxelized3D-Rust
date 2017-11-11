extern crate cc;

fn main() {
    cc::Build::new()
        .file("./src/C/util.c")
        .file("./src/C/glad.c")
        .include("./src/H")
        .include("include")
        .compile("rsutil");
}
