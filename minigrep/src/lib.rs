use std::fs;
use std::error::Error;
use std::env;

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub sensitive: bool,
}

impl <'a> Config <'a>{
/*
    fn new(args: &[String]) -> Config
    {
        if args.len() < 3 {
            panic!(" ---> Not enough arguments! <--- ");
        }
        let query = &args[1];
        let filename = &args[2];

        Config {query, filename}
    }
*/
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough parameters!");
        }

        let query = &args[1];
        let filename = &args[2];
        let sensitive = env::var("CASE_SENSITIVE").is_err();
        println!("sensitive: {}", sensitive);
        Ok(Config {query, filename, sensitive})
    }

    pub fn open_file(&self) -> Result<String, &'static str> {
        let content = fs::read_to_string(self.filename).expect("could not open file!");
        return Ok(content);
    }
}

pub fn run(config: &Config) -> Result<(), &'static str>{
    match config.open_file() {
        Ok(contents) => {
            if config.sensitive {
                for line in search_case_sensitive(&config.query, &contents) {
                    println!("{}", line);
                }
            }
            else {
                for line in search_case_insensitive(&config.query, &contents) {
                    println!("{}", line);
                }
            }
        }
        Err(e) => return Err(e)
    };

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    return results;
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }

    return results;
}
