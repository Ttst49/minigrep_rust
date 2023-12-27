use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args : Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problème rencontré lors de l'interprétation des arguments : {}", err);
        process::exit(1);
    });

    println!("On recherche : {}", config.search_arg);
    println!("Dans le fichier : {}", config.file_name);

    if let Err(e) = run(config) {
        println!("Erreur applicative : {}", e);

        process::exit(1);
    }
}

struct Config{
    search_arg: String,
    file_name: String
}

impl Config {
    fn new(args : &[String])->Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Il n'y a pas assez d'arguments")
        }

        let search_arg = args[1].clone();
        let file_name = args[2].clone();

        Ok(Config{search_arg, file_name})
    }
}

fn run( config: Config)-> Result<(),Box<dyn Error>>{
    let content = fs::read_to_string(config.file_name)?;


    println!("le contenu est : '{}'",content);

    Ok(())
}