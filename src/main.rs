fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = minigrep::Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    minigrep::run(config).unwrap_or_else(|err|{
        println!("Application error: {err}");
        std::process::exit(1);
    });
}
