use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(),  Box<dyn Error>> /*return any type of Error*/ {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    println!("---------------------");

    if results.is_empty() {
        println!("{:?} is not any of the lines!!",&config.query);
    } else {
        println!("The lines are :");
        println!("{:#?}",results);
    }

    println!("---------------------");

    println!("The contents were :");
    for line in contents.lines() {
        println!("{}",line);
    }

    Ok(())
}

// fn run(config: Config)  {
//     let contents = fs::read_to_string(config.filename)
//         .expect("Not enough arguments!");
//     println!("With text: \n{}", contents);
// }

pub struct  Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}



impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return  Err("not enough arguments");
        }

        let query =  args[1].clone();
        let filename = args[2].clone(); 
        //In bash ------> export CASE_INSENSITIVE=true
        // unset
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {query, filename, case_sensitive})
    }
}

//Here lifetime of contents is tied to the output strings
//Because the output lines are simply the lines from contents
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    // let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "Duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three
Duct tape.";
        assert_eq!(vec!["Duct tape."],search(query, contents));
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

















