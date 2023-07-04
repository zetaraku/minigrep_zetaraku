fn main() {
    let args: Vec<String> = std::env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for `{query}` in file `{file_path}`");


    let contents = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
