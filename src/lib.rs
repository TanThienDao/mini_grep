pub fn search<'a>(query: &str, contents: &'a str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line.to_string());
        }
    }
    result
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<String> {
    let query = query.to_lowercase();
    let mut result: Vec<String> = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line.to_string());
        }
    }
    result
}

#[cfg(test)]mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust :\nsafe, fast, productive.\nPick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_sensitive() {
        let query = "duck";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuck tape.";
        assert_ne!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}