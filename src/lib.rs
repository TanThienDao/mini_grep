pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut result: Vec<&str> = Vec::new();
    for line in contents.lines() {
        println!("line: {}", line);
        println!("check: {}", line.contains(query));
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust :\
        safe, fast, productive.\
        Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));

    }
}