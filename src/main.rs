fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = parse_config(&args);

    println!(
        "Searching for `{query}` in file `{file_path}`",
        query = config.query,
        file_path = config.file_path,
    );

    let contents = std::fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
