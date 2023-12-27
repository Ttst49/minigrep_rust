use std::env;
use std::fs;

fn main() {
    let args : Vec<String> = env::args().collect();

    let (search_arg, file_name) = interpreter_config(&args);

    println!("On recherche : {}", search_arg);
    println!("Dans le fichier : {}", file_name);

    let content = fs::read_to_string(file_name)
        .expect("Quelque chose a échoué");

    println!("le contenu de {} est : '{}'",file_name,content)
}

struct Config{
    search_arg: String,
    file_name: String
}

fn interpreter_config(args : &[String])->Config{
    let search_arg = args[1].clone();
    let file_name = args[2].clone();

    Config{search_arg, file_name}
}