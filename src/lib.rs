use std::error::Error;
use std::fs;

pub struct Config {
    target: String,
    file_path: String,
    show_line_number: bool,
    ignore_case: bool,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let config = Config {
            target: args[0].clone(),
            file_path: args[1].clone(),
            show_line_number: args.contains(&"-n".to_string()),
            ignore_case: args.contains(&"-i".to_string()),
        };
        Ok(config)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let result = match config.ignore_case {
        true => search_ignore_case(&config.target, &contents),
        false => search_case_sensitive(&config.target, &contents),
    };

    let _ = match config.show_line_number {
        true => {
            for (i, line) in result.iter() {
                println!("{i}:{line}");
            }
        }
        false => {
            for (_i, line) in result.iter() {
                println!("{line}");
            }
        }
    };
    Ok(())
}

pub fn search_case_sensitive<'a>(target: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let mut results = Vec::new();
    for (i, line) in contents.lines().enumerate() {
        if line.contains(target) {
            results.push((i + 1, line));
        }
    }
    results
}

pub fn search_ignore_case<'a>(target: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let mut results = Vec::new();
    let target = target.to_lowercase();
    for (i, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&target) {
            results.push((i + 1, line));
        }
    }
    results
}

// TDD: Test Driven Development
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let target = "less";
        let contents = "\
export EDITOR=nvim
export VISUAL=nvim
export PAGER=less";
        assert_eq!(
            vec![(3, "export PAGER=less")],
            search_case_sensitive(target, contents)
        );
        let target = "nvim";
        assert_eq!(
            vec![(1, "export EDITOR=nvim"), (2, "export VISUAL=nvim")],
            search_case_sensitive(target, contents)
        );
    }

    #[test]
    fn ignore_case() {
        let target = "LeSs";
        let contents = "\
export EDITOR=nvim
export VISUAL=nvim
export PAGER=less";
        assert_eq!(
            vec![(3, "export PAGER=less")],
            search_ignore_case(target, contents)
        );
    }
}
