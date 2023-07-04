fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = Config::new(&args);

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

impl Config {
    fn new(args: &[String]) -> Self {
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Self { query, file_path }
    }
}
