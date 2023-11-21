use std::env;
use colored::{*};
fn main() {
    let text="
███████╗██████╗ ███████╗███████╗
██╔════╝██╔══██╗██╔════╝██╔════╝
█████╗  ██████╔╝█████╗  █████╗
██╔══╝  ██╔══██╗██╔══╝  ██╔══╝
██║     ██║  ██║███████╗███████╗
╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝

 █████╗ ███████╗███████╗ █████╗ ███╗   ██╗ ██████╗ ███████╗
██╔══██╗██╔════╝██╔════╝██╔══██╗████╗  ██║██╔════╝ ██╔════╝
███████║███████╗███████╗███████║██╔██╗ ██║██║  ███╗█████╗
██╔══██║╚════██║╚════██║██╔══██║██║╚██╗██║██║   ██║██╔══╝
██║  ██║███████║███████║██║  ██║██║ ╚████║╚██████╔╝███████╗
╚═╝  ╚═╝╚══════╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝ ╚══════╝
".red().bold();
println!("{}",text);

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Veuillez taper une chaîne de caractères comme argument.");
        return;
    }
    let input = &args[1];
    let reversed: String = input.chars().rev().collect();
    println!("Chaîne inversée : {}", reversed);
    
 // OU ...

    let f = maReverse(&reversed);
    println!("Reinversion:{}",f);
   
    println!("{}","------------------------------:".red());
}


fn maReverse(chaine: &str) -> String {
    let mut rev:String = String::new();
    for ch in chaine.chars().rev(){ // chaine.chars().rev() crée un itérateur qui parcourt les caractères de la chaîne chaine en sens inverse.
        rev.push(ch) // rempli la chaine. one by one.
    }
    rev
}