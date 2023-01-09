fn main() {
    println!(
        "libc: {}",
        std::process::Command::new("ldd")
            .arg("--version")
            .output()
            .unwrap_or_else(|_| std::process::exit(1))
            .stdout
            .escape_ascii()
    );

   let bt =  backtrace::Backtrace::new();

   println!("{:?}", bt);
}
