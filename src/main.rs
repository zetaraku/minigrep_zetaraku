fn main() {
    let config = minigrep_zetaraku::Config::build(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    minigrep_zetaraku::run(config).unwrap_or_else(|err|{
        eprintln!("Application error: {err}");
        std::process::exit(1);
    });
}
