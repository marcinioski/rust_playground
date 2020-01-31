#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.";

        assert_eq!(vec!["safe, fast, productive."],
            minigrep::search_case_sensitive(query, contents));
    }

    #[test]
    fn search_case_insensitive() {
        let query = "DuCt";
        let contents = "\
Rust:
safe, fast, productive.";
        assert_eq!(vec!["safe, fast, productive."],
            minigrep::search_case_insensitive(query, contents));
    }
}
