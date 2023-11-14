use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Veuillez fournir une heure en format 24h (HH:MM).");
        return;
    }
    let time = &args[1];
    let parts: Vec<&str> = time.split(':').collect();
    if parts.len() != 2 {
        println!("Format invalide. Utilisez le format HH:MM.");
        return;
    }
    let hour: i32 = match parts[0].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Heure invalide.");
            return;
        }
    };

    let minute: i32 = match parts[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Minute invalide.");
            return;
        }
    };

    convert_to_12h_format(hour, minute);
}
fn convert_to_12h_format(hour: i32, minute: i32) {
    if hour < 0 || hour > 23 || minute < 0 || minute > 59 {
        println!("Heure ou minute hors de portée.");
        return;
    }

    let (formatted_hour, am_pm) = if hour == 0 {
        (12, "AM")
    } else if hour == 12 {
        (12, "PM")
    } else if hour < 12 {
        (hour, "AM")
    } else {
        (hour - 12, "PM")
    };

    println!("{:02}:{:02}{}", formatted_hour, minute, am_pm);
    alice();
}

fn alice(){
    println!("

                             /$$$$$$
                            /$$$_  $$
  /$$$$$$ /$$   /$$ /$$$$$$| $$$$\\ $$/$$$$$$$$
 /$$__  $|  $$ /$$//$$__  $| $$ $$ $|____ /$$/
| $$$$$$$$\\  $$$$/| $$  \\ $| $$\\ $$$$  /$$$$/
| $$_____/ >$$  $$| $$  | $| $$ \\ $$$ /$$__/
|  $$$$$$$/$$/\\  $| $$$$$$$|  $$$$$$//$$$$$$$$
 \\_______|__/  \\__| $$____/ \\______/|________/
                  | $$
                  | $$
                  |__/

    ");
}