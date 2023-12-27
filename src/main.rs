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


fn interpreter_config(args : &[String])->(&str, &str){
    let search_arg = &args[1];
    let file_name = &args[2];

    (search_arg, file_name)
}