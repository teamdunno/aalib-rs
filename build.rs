#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    println!("cargo:rustc-link-lib=gpm");
    println!("cargo:rustc-link-lib=slang");
}
