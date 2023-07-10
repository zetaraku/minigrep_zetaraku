//! # minigrep_zetaraku
//!
//! `minigrep_zetaraku` is a collection of utilities used to search for lines containing a specific string.

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        // skip program name
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = std::env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

/// Search for lines in `contents` containing `query`.
///
/// # Examples
///
/// ```
/// let query = "duct";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Duct tape.";
///
/// assert_eq!(vec!["safe, fast, productive."], minigrep_zetaraku::search(query, contents));
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line: &&str| line.contains(query))
        .collect()
}

/// Search for lines in `contents` containing `query` (case insensitive).
///
/// # Examples
///
/// ```
/// let query = "rUsT";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Trust me.";
///
/// assert_eq!(vec!["Rust:", "Trust me."], minigrep_zetaraku::search_case_insensitive(query, contents));
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let lowercase_query = query.to_lowercase();

    contents
        .lines()
        .filter(|line: &&str| line.to_lowercase().contains(&lowercase_query))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents),
        );
    }
}
