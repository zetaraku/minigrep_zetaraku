fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = minigrep::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    minigrep::run(config).unwrap_or_else(|err|{
        eprintln!("Application error: {err}");
        std::process::exit(1);
    });
}
