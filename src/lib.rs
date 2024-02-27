use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(String::from("not enough arguments"));
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = match env::var("IGNORE_CASE") {
            Ok(s) => {
                if s == "1" {
                    true
                } else {
                    false
                }
            }

            Err(_) => false,
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for result in results {
        println!("{result}");
    }

    // println!("With text:\n{}", contents);

    Ok(())
}

/**
# 测试驱动开发
*/
#[cfg(test)]
mod tests {
    #[test]

    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            crate::search(query, contents)
        );
    }
    #[test]
    fn two_result() {
        let query = "rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["Rust:"],
            crate::search_case_insensitive(query, contents)
        );
    }
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}
