use std::error::Error;
use std::fs;

pub struct Config{
    pub search_arg: String,
    pub file_name: String
}

impl Config {
    pub fn new(args : &[String])->Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Il n'y a pas assez d'arguments")
        }

        let search_arg = args[1].clone();
        let file_name = args[2].clone();

        Ok(Config{search_arg, file_name})
    }
}

pub fn run( config: Config)-> Result<(),Box<dyn Error>>{
    let content = fs::read_to_string(config.file_name)?;

    for line in search_content(&config.search_arg, &content) {
        println!("{}",line)
    }

    Ok(())
}

pub fn search_content<'a>(search : &str, content: &'a str)-> Vec<&'a str>{
    let mut result = Vec::new();

    for line in content.lines() {
        if line.contains(search) {
            result.push(line)
        }
    }

    result
}


pub fn search_content_not_case_sensitive<'a>(search: &str, content: &'a str)->Vec<&'a str>{
    let search = search.to_lowercase();
    let mut result = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&search) {
            result.push(line)
        }
    }
    result
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    #[ignore]
    fn case_sensitive(){
        let search = "duct";
        let content = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.
Duck tape.";

        assert_eq!(
            vec!["sécurité, rapidité, productivité."],
            search_content(search, content)
        );
    }

    #[test]
    fn not_case_sensitive(){
        let search = "rUsT";
        let content = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.
C'est pas rustique.";

        assert_eq!(
            vec!["Rust:", "C'est pas rustique."],
            search_content_not_case_sensitive(search, content)
        );
    }

}