use std::env;
fn main() {
    println!("███╗   ██╗ ██████╗     ██╗ █████╗");
    println!("████╗  ██║██╔═══██╗    ██║██╔══██╗");
    println!("██╔██╗ ██║██║   ██║    ██║███████║");
    println!("██║╚██╗██║██║   ██║    ██║██╔══██║");
    println!("██║ ╚████║╚██████╔╝    ██║██║  ██║");
    println!("╚═╝  ╚═══╝ ╚═════╝     ╚═╝╚═╝  ╚═╝");

    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 3 {
        println!("Veuillez taper deux entiers comme arguments.");
        return;
    }
    let i = match arguments[1].parse::<i32>() {
        Ok(val) => val,
        Err(_) => {
            println!("Le premier argument n'est pas un entier valide.");
            return;
        }
    };

    let j = match arguments[2].parse::<i32>() {
        Ok(val) => val,
        Err(_) => {
            println!("Le deuxième argument n'est pas un entier valide.");
            return;
        }
    };
    println!("La division est : {}", i / j);
    println!("Le modulo est : {}", i % j);
    println!("---");
}
