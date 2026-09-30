fn main() {
    if let Err(error) = xtask::execution::cli::run_cli::run_cli() {
        eprintln!("{error:#}");
        std::process::exit(1);
    }
}
