use super::*;

static CONTENTS: &str = "Rust:\nsafe, fast, productive.\nPick three.";

#[test]
fn case_sensitive() {
    let query = "duct";
    assert_eq!(vec!["safe, fast, productive."], search(query, CONTENTS));
}

#[test]
fn case_insensitive() {
    let query = "rust";
    assert_eq!(vec!["Rust:"], search_case_insensitive(query, CONTENTS))
}
