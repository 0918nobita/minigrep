//! # Minigrep
//! command line tool used to search for lines in a specified file

use std::fs;
use std::io;

pub mod config;
use config::Config;

#[cfg(test)]
mod test;

pub fn run(config: &Config) -> io::Result<()> {
    let contents = fs::read_to_string(config.filename.as_str())?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    results.for_each(|line| println!("{}", line));

    Ok(())
}

fn search<'a>(query: &'a str, contents: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> {
    Box::new(contents.lines().filter(move |line| line.contains(query)))
}

fn search_case_insensitive<'a>(
    query: &'a str,
    contents: &'a str,
) -> Box<dyn Iterator<Item = &'a str> + 'a> {
    let query = query.to_lowercase();

    Box::new(
        contents
            .lines()
            .filter(move |line| line.to_lowercase().contains(&query)),
    )
}
