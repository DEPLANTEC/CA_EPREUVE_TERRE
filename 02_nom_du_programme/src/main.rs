use std::env;

fn main() {
    println!(" #### PROGRAM STARTING --- > ");

    let args: Vec<String> = env::args().collect();
    let program_path = &args[0];
    let components: Vec<&str> = program_path.split('\\').collect();
    let program_name = components.last().unwrap_or(&"Unknown");

    println!("Nom du Programme : {}", program_name);
    println!(" <--- TERMINATED #### ");
}
