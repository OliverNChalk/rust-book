use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new<T>(mut args: T) -> Result<Config, &'static str>
    where
        T: Iterator<Item = String>,
    {
        args.next(); // usually arg1 is the executable, skip it

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("missing query argument"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("missing filename argument"),
        };
        if let Some(_) = args.next() {
            return Err("additional arguments");
        }

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    return contents
        .lines()
        .filter(|line| line.contains(query))
        .collect();
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    return contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_insufficient_args() {
        // let args = [String::from("/path/to/bin"), String::from("arg1")];
        let args = [String::from("/path/to/bin"), String::from("arg1")];
        let config = Config::new(IntoIterator::into_iter(args));

        assert!(config.is_err());
    }

    #[test]
    fn accepts_exact_args() {
        let args = [
            String::from("/path/to/bin"),
            String::from("arg1"),
            String::from("arg2"),
        ];
        let config = Config::new(IntoIterator::into_iter(args));

        assert!(config.is_ok());
    }

    #[test]
    fn rejects_additional_args() {
        let args = [
            String::from("/path/to/bin"),
            String::from("arg1"),
            String::from("arg2"),
            String::from("arg3"),
        ];
        let config = Config::new(IntoIterator::into_iter(args));

        assert!(config.is_err());
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
