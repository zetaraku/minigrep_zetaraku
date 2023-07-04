fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    println!(
        "Searching for `{query}` in file `{file_path}`",
        query = config.query,
        file_path = config.file_path,
    );

    run(config).unwrap_or_else(|err|{
        println!("Application error: {err}");
        std::process::exit(1);
    });
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Self { query, file_path })
    }
}
