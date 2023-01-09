fn main() {

    println!("libc: {}", std::process::Command::new("ldd").arg("--version").output().unwrap().stdout.escape_ascii());
}
