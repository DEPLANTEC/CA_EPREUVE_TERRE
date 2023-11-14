use std::env;

fn main() {
    print!("
███████╗██╗  ██╗██████╗  ██████╗ ███████╗
██╔════╝╚██╗██╔╝██╔══██╗██╔═████╗╚══███╔╝
█████╗   ╚███╔╝ ██████╔╝██║██╔██║  ███╔╝
██╔══╝   ██╔██╗ ██╔═══╝ ████╔╝██║ ███╔╝
███████╗██╔╝ ██╗██║     ╚██████╔╝███████╗
╚══════╝╚═╝  ╚═╝╚═╝      ╚═════╝ ╚══════╝
    ");
    println!("");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Veuillez taper une chaîne de caractères comme argument.");
        return;
    }
    let input = &args[1];
    let length = input.chars().count();
    println!("Le nombre de caractères : {}", length);
}
