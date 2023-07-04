fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = minigrep::Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    println!(
        "Searching for `{query}` in file `{file_path}`",
        query = config.query,
        file_path = config.file_path,
    );

    minigrep::run(config).unwrap_or_else(|err|{
        println!("Application error: {err}");
        std::process::exit(1);
    });
}
