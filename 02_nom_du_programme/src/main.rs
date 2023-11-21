use std::env;

fn main() {

    println!(" --- > ");

    let args: Vec<String> = env::args().collect();
    let program_path = &args[0];
    let components: Vec<&str> = program_path.split('\\').collect();
    let program_name = components.last().unwrap_or(&"Unknown");
    println!("Program name : {}", program_name);

    println!(" <--- ");
}
