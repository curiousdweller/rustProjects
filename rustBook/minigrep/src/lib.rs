use std::error::Error;
use std::{fs, vec};
use std::env;

pub struct Config {
    pub filename: String,
    pub data: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new<T>(mut args: T) -> Result<Config, &'static str> 
    where T: Iterator<Item = String>,
    {
        args.next();
        let data = match args.next() {
            Some(s) => s,
            None => return Err("Error configuring data to search")
        };

        let filename = match args.next() {
            Some(s) => s,
            None => return Err("Did not get file name")
        };

        let case_sensitive = env::var("CASE_SENSITIVE").is_err();
        println!("{}", case_sensitive);
        Ok(Config {
            filename,
            data,
            case_sensitive
        })


    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    if config.case_sensitive {
        for line in search(&config.data, &contents) {
            println!("\x1b[94m{}\x1b[0m", line);
        }
    } else{
        for line in case_insensitive_search(&config.data, &contents) {
            println!("\x1b[94m{}\x1b[0m", line);
        }
    }

        Ok(())

}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|x | x.contains(query)).collect()
}
fn case_insensitive_search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    for line in content.lines() {
        if line.to_lowercase().contains(query.to_lowercase().as_str()) {
            results.push(line);
        }
    }
    results
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_test() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], super::search(query, content ))
    }

    #[test]
    fn case_insensitive_search_test() {
        let query = "ruST";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], case_insensitive_search(query, contents));
    }


    
}