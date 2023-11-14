use std::{thread, time};
use colored::{*};

fn main() {
    println!(" Welcome....");
    let delay = time::Duration::from_millis(2000);
    thread::sleep(delay);
    let phrases = [
        "Chargement des données...",
        "Analyse de l'épreuve...",
        "Compilation des résultats...",
        "Préparation de la célébration...",
        "J'ai terminé l'Épreuve de la Terre et c'était épique."
    ];

    for phrase in phrases.iter() {
        print!("\r{}", phrase);
        let delay = time::Duration::from_millis(100);
        thread::sleep(delay);
        println!();
    }
    println!("{}","100% Transfert DATA Enjoy! THX-TEAM ".white().on_red().bold());
}
