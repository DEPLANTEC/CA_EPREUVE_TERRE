
use std::env;

fn main() {
    println!("");
    println!("#>-----------------------------------------------------<()>");

    let _args:Vec<String> = env::args().collect();
    for i in 1.._args.len(){
        println!("- {}: {}",i,&_args[i])
    }
    println!("#>-----------------------------------------------------<()>");
    println!("");
}