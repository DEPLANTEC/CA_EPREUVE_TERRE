use std::env;

fn main() {
    println!("---[");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Veuillez fournir un entier comme argument.");
        //panic!(); // Désagreable !
        return;
    }
    match args[1].parse::<i32>() {
        Ok(n) => {
            if n % 2 == 0 {
                println!("{} est pair.", n);
            } else {
                println!("{} est impair.", n);
            }
        }
        Err(_) => {
            println!("Veuillez fournir un entier valide.");
        }
    }
    println!("]---");
}
