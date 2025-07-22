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
    let result = search(&config.target, &contents, config.ignore_case);

    for (i, line) in result.iter() {
        if config.show_line_number {
            println!("{i}:{line}");
        } else {
            println!("{line}");
        }
    }
    Ok(())
}

pub fn search<'a>(target: &str, contents: &'a str, ignore_case: bool) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            if ignore_case {
                line.to_lowercase().contains(&target.to_lowercase())
            } else {
                line.contains(target)
            }
        })
        .map(|(i, line)| (i + 1, line))
        .collect()
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
            search(target, contents, false)
        );
        let target = "nvim";
        assert_eq!(
            vec![(1, "export EDITOR=nvim"), (2, "export VISUAL=nvim")],
            search(target, contents, false)
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
            search(target, contents, true)
        );
    }
}
