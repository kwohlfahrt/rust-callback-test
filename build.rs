extern crate gcc;

fn main() {
    gcc::Config::new()
        .file("src/c/callback.c")
        .include("src")
        .compile("libcallback.a");
}
