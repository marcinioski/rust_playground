use std::fs;
use std::error::Error;

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
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

        Ok(Config {query, filename})
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    return results;
}

