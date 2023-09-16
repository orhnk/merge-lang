extern crate cc;

fn main() {
    cc::Build::new().file("src/pipe.c").compile("pipe");
}
