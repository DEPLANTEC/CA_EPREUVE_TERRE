use std::env;

fn main() {
    println!("---( STARTED )---");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Veuillez taper une lettre minuscule comme argument.");
        return;
    }
    let start_char = args[1].chars().next().unwrap_or_default();
    if !start_char.is_ascii_lowercase() {
        println!("Veuillez taper une lettre minuscule valide.");
        return;
    }
    for c in start_char..='z' {
        print!("{}", c);
    }
    println!();
    println!("---( TERMINATED )---");
}
