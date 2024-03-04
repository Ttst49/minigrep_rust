use std::error::Error;
use std::fs;
use std::env;

pub struct Config{
    pub search_arg: String,
    pub file_name: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(mut args : env::Args)->Result<Config, &'static str> {
        args.next();

        if args.len() < 3 {
            return Err("Il n'y a pas assez d'arguments")
        }

        let search_arg = match args.next() {
            Some(arg) => arg,
            None=> return Err("Pas de chaine de caractère présente")
        };

        let file_name = match args.next() {
            Some(arg)=>arg,
            None=> return Err("Pas de nom de fichier présent")
        };

        let case_sensitive = env::var("MINIGREP_CASE_SENSITIVE").is_err();

        Ok(Config{search_arg, file_name,case_sensitive})
    }
}

pub fn run( config: Config)-> Result<(),Box<dyn Error>>{
    let content = fs::read_to_string(config.file_name)?;

    let results = if config.case_sensitive {
        search_content(&config.search_arg, &content)
    } else {
        search_content_not_case_sensitive(&config.search_arg, &content)
    };

    for line in results {
        println!("{}",line)
    }

    Ok(())
}

pub fn search_content<'a>(search : &str, content: &'a str)-> Vec<&'a str>{
    content
        .lines()
        .filter(|line| line.contains(search))
        .collect()
}


pub fn search_content_not_case_sensitive<'a>(search: &str, content: &'a str)->Vec<&'a str>{
    content
        .lines()
        .filter(|line| line.contains(search.to_lowercase()))
        .collect()
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