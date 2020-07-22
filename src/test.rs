use super::*;

static CONTENTS: &str = "Rust:\nsafe, fast, productive.\nPick three.";

#[test]
fn case_sensitive() {
    let query = "duct";
    let lines: Vec<_> = search(query, CONTENTS).collect();
    assert_eq!(vec!["safe, fast, productive."], lines);
}

#[test]
fn case_insensitive() {
    let query = "rust";
    let lines: Vec<_> = search_case_insensitive(query, CONTENTS).collect();
    assert_eq!(vec!["Rust:"], lines);
}
