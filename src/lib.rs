use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(value) => value,
            None => return Err("missing parameter query"),
        };

        let file_path = match args.next() {
            Some(value) => value,
            None => return Err("missing parameter file path"),
        };

        Ok(Config {
            query: query.to_string(),
            file_path: file_path.to_string(),
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(&config.file_path)?;

    let matching_lines = search(&config.query, &file_content);

    for line in matching_lines {
        println!("{}", formatt_line(&config.query, line))
    }

    Ok(())
}

fn formatt_line(query: &str, line: &str) -> String {
    let line_formatted = line
        .split(" ")
        .map(|word| {
            if word.to_lowercase().contains(&query.to_lowercase()) {
                return "**".to_string() + word + "**" + " ";
            }

            word.to_string() + " "
        })
        .collect();

    line_formatted
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let matching_lines: Vec<&str> = contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect();

    matching_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_returns_lines_that_match_the_query() {
        let query = "simpler";
        let contents = "\
Pennant: A small flag, often used as a dymbol.
Condense: To make something shorter or simpler.
Meliorate: To improve or make something better.";

        assert_eq!(
            vec!["Condense: To make something shorter or simpler."],
            search(query, contents)
        );
    }

    #[test]
    fn search_returns_0_lines_if_query_doesnt_match() {
        let query = "rustacean";
        let contents = "\
Pennant: A small flag, often used as a dymbol.
Condense: To make something shorter or simpler.
Meliorate: To improve or make something better.";

        let empty_vec: Vec<&str> = vec![];
        assert_eq!(empty_vec, search(query, contents));
    }
}
