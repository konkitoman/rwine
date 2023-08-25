pub fn main() {
    println!("cargo:rustc-link-search=./.xwin-cache/splat/sdk/lib/um/x86_64");
    println!("cargo:rustc-link-search=./.xwin-cache/splat/sdk/lib/ucrt/x86_64");
    println!("cargo:rustc-link-search=./.xwin-cache/splat/crt/lib/x86_64");
}
