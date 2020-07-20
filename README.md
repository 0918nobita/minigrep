# minigrep

command line tool used to search for lines in a specified file

## Usage

```
$ cargo run [search string] [filename]
```

### Environment variables

- `CASE_INSENSITIVE=1` enables case-insensitive search

example:

```bash
$ cargo run safe input.txt
safe, fast, productive.

$ CASE_INSENSITIVE=1 cargo run rust input.txt
Rust:
```
