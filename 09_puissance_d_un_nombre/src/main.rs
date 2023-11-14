use std::env;
use std::num::ParseIntError;
fn main() {
    println!("Program Starting !!");
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Veuillez taper deux arguments : la base et l'exposant non négatif.");
        return;
    }
    let base = parse_arg(&args[1]);
    let exponent = parse_arg(&args[2]);
    match (base, exponent) {
        (Ok(base), Ok(exp)) if exp >= 0 => {
            let result = base.pow(exp as u32); // Convertir l'exposant en u32 pour pow
            println!("Le résultat de {}^{} est : {}", base, exp, result);
        }
        _ => {
            print!("
                        ███████╗██╗   ██╗ ██████╗██╗     ██╗██████╗ ███████╗
                        ██╔════╝██║   ██║██╔════╝██║     ██║██╔══██╗██╔════╝
                        █████╗  ██║   ██║██║     ██║     ██║██║  ██║█████╗
                        ██╔══╝  ██║   ██║██║     ██║     ██║██║  ██║██╔══╝
                        ███████╗╚██████╔╝╚██████╗███████╗██║██████╔╝███████╗
                        ╚══════╝ ╚═════╝  ╚═════╝╚══════╝╚═╝╚═════╝ ╚══════╝
");
            println!("Veuillez taper des arguments valides. L'exposant ne doit pas être négatif.");
            println!("");
        }
    }
    println!("Terminated !!");
}

fn parse_arg(arg: &str) -> Result<i32, ParseIntError> {
    arg.parse::<i32>()
}
    