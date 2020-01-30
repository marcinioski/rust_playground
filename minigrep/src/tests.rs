#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.";

        assert_eq!(vec!["safe, fast, productive."],
            minigrep::search(query, contents));
    }
}
