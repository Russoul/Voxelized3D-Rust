extern crate cc;

fn main() {
    println!("cargo:rustc-link-search=native={}/lib",  "glad");
}


