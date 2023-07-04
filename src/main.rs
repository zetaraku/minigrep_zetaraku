fn main() {
    let args: Vec<String> = std::env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for `{query}` in file `{file_path}`");
}
