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


    println!("le contenu est : '{}'",content);

    Ok(())
}

pub fn search_content<'a>(search : &str, content: &str)-> Vec<&'a str>{
    vec![]
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let search = "duct";
        let content = "\
            Rust:
            sécurité, rapidité, productivité.
            Obtenez les trois en même temps.";

        assert_eq!(
            vec!["sécurité, rapidité, productivité."],
            search_content(search, content)
        );
    }

}