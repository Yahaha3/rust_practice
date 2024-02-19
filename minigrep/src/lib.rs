
use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config{query, file_path, ignore_case})
    }

    pub fn build1(mut arg: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        arg.next();

        let query = match arg.next() {
            Some(a) => a,
            None => return Err("Didn't get a query string"),
        }; 

        let file_path = match arg.next() {
            Some(a) => a,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    // println!("With text: \n{contents}");
    let result = if config.ignore_case {
        search_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
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
        let query = "rUst";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_insensitive(query, contents));
    }
}

pub fn search<'a>(query: &str, contents: &'a str) ->Vec<&'a str>{
    // let mut result = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //             result.push(line.trim());
    //     }
    // }

    // result
    contents.lines().filter(|line| line.contains(query)).map(|x| x.trim()).collect()

}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) ->Vec<&'a str> {
    let query = query.to_lowercase();
    // let mut result = Vec::new();

    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         result.push(line.trim());
    //     }
    // }

    // result

    contents.lines().filter(|line| line.to_lowercase().contains(&query)).map(|x| x.trim()).collect()
}