use std::env;
use colored::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Veuillez fournir une heure en format 12h (HH:MMAM ou HH:MMPM).");
        return;
    }

    let time = &args[1];
    let parts: Vec<&str> = time.split(':').collect();
    if parts.len() != 2 || parts[1].len() < 4 {
        println!("Format invalide. Utilisez le format HH:MMAM ou HH:MMPM.");
        return;
    }
    let hour: i32 = match parts[0].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Heure invalide.");
            return;
        }
    };
    let minutes_and_period = &parts[1];
    let minute: i32 = match minutes_and_period[..2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Minute invalide.");
            return;
        }
    };
    let period = &minutes_and_period[2..];
    convert_to_24h_format(hour, minute, period);
}

fn convert_to_24h_format(hour: i32, minute: i32, period: &str) {
    if hour < 1 || hour > 12 || minute < 0 || minute > 59 || (period != "AM" && period != "PM") {
        println!("Format de l'heure invalide.");
        return;
    }

    let formatted_hour = if hour == 12 {
        match period {
            "AM" => 0, // Minuit
            "PM" => 12, // Midi
            _ => hour,
        }
    } else {
        match period {
            "AM" => hour,
            "PM" => hour + 12,
            _ => hour,
        }
    };

    println!("{:02}:{:02}", formatted_hour, minute);
    println!("{}","
@@@@@@@@  @@@  @@@  @@@@@@@    @@@@@@@@   @@@@@@@@
@@@@@@@@  @@@  @@@  @@@@@@@@  @@@@@@@@@@  @@@@@@@@
@@!       @@!  !@@  @@!  @@@  @@!   @@@@       @@!
!@!       !@!  @!!  !@!  @!@  !@!  @!@!@      !@!
@!!!:!     !@@!@!   @!@@!@!   @!@ @! !@!     @!!
!!!!!:      @!!!    !!@!!!    !@!!!  !!!    !!!
!!:        !: :!!   !!:       !!:!   !!!   !!:
:!:       :!:  !:!  :!:       :!:    !:!  :!:
 :: ::::   ::  :::   ::       ::::::: ::   :: ::::
: :: ::    :   ::    :         : : :  :   : :: : :


    ".red());
}
