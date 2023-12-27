use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();
    let search_arg = &args[1];
    let file_name = &args[2];

    println!("On recherche : {}", search_arg);
    println!("Dans le fichier : {}", file_name);
}
