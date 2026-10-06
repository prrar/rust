fn main() {
    if let Err(e) = prcat::run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
