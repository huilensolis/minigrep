use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            if args.len() < 2 {
                return Err("missing search query and a file path parameters. \nexample: cargo run search_query path/to/my/file.txt");
            }

            return Err("must receive a file path parameter. \nexample: cargo run search_query path/to/my/file.txt");
        }

        let query = &args[1].clone();

        let file_path = &args[2].clone();

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
    let mut line_formatted = String::from("");

    let line_words = line.split(" ");

    for word in line_words {
        if word.to_lowercase().contains(&query.to_lowercase()) {
            let highlighted_word = "**".to_string() + word + "** ";
            line_formatted.push_str(&highlighted_word);
            continue;
        }

        let word_with_spaces_on_sides = word.to_string() + " ";
        line_formatted.push_str(&word_with_spaces_on_sides)
    }

    line_formatted
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matching_lines: Vec<&str> = vec![];

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.trim().to_lowercase()) {
            matching_lines.push(line)
        }
    }

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
