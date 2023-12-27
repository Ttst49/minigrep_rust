use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args : Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problème rencontré lors de l'interprétation des arguments : {}", err);
        process::exit(1);
    });

    println!("On recherche : {}", config.search_arg);
    println!("Dans le fichier : {}", config.file_name);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Erreur applicative : {}", e);

        process::exit(1);
    }
}
