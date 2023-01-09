fn main() {
    let version = unsafe { libc::gnu_get_libc_version() };
    let str = unsafe { std::ffi::CStr::from_ptr(version) };
    println!("cargo:warning=libc: {:?}", str);
    println!("cargo:rerun-if-changed=build.rs");
}
