fn main() {
    let config = minigrep::Config::build(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    minigrep::run(config).unwrap_or_else(|err|{
        eprintln!("Application error: {err}");
        std::process::exit(1);
    });
}
