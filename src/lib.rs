/*pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}*/
/// new search improve function :
pub fn search<'a>(
    query: &str,
    contents: &'a str
) -> impl Iterator<Item = &'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect::<Vec<&str>>()
        .into_iter()
}
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> impl Iterator<Item = &'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect::<Vec<&str>>()
        .into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust :\nsafe, fast, productive.\nPick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents).collect::<Vec<&str>>()
        );
    }
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents).collect::<Vec<&str>>()
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents).collect::<Vec<&str>>()
        );
    }
}
