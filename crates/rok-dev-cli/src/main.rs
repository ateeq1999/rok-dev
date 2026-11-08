//! The `rok-dev` binary.

fn main() {
    std::process::exit(rok_dev_cli::run_from(std::env::args_os()));
}
