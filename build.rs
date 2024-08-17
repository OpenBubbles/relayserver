
extern crate cc;
fn main() {

    println!("cargo:rustc-link-lib=dylib=MobileGestalt");

    cc::Build::new()
        .archiver("/home/tae/theos/toolchain/linux/iphone/bin/ar")
        .file("src/c/relay.c")
        .file("src/c/absdUser.c").compile("relay");
}