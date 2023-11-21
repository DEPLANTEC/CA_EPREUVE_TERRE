use std::env;

fn main() {
    print!("
███████╗██╗  ██╗██████╗  ██████╗ ███████╗
██╔════╝╚██╗██╔╝██╔══██╗██╔═████╗╚══███╔╝
█████╗   ╚███╔╝ ██████╔╝██║██╔██║  ███╔╝
██╔══╝   ██╔██╗ ██╔═══╝ ████╔╝██║ ███╔╝
███████╗██╔╝ ██╗██║     ╚██████╔╝███████╗
╚══════╝╚═╝  ╚═╝╚═╝      ╚═════╝ ╚══════╝
    "); println!("");

 
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { println!("Veuillez taper une chaîne de caractères comme argument.");
        return;
    }
    let input = &args[1];
    let length = input.chars().count();

    println!("Nb de caractères (v1): {}", length);
    // OU ..
    println!("Nb de caractères (v2): {}", mafonction(input));
}

fn mafonction(input: &str) -> usize {
    input.chars().filter(|c| c.is_alphabetic()).count()
}