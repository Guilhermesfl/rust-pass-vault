mod pentry;

use crate::pentry::prompt;
use crate::pentry::read_passwords_from_file;

fn clr() {
    print!("{}[2j", 27 as char);
}

fn main() {
    clr();
    let ascii = r#"        
        __________                __                                     
        \______   \__ __  _______/  |______    ____  ____ _____    ____  
        |       _/  |  \/  ___/\   __\__  \ _/ ___\/ __ \\__  \  /    \ 
        |    |   \  |  /\___ \  |  |  / __ \\  \__\  ___/ / __ \|   |  \
        |____|_  /____//____  > |__| (____  /\___  >___  >____  /___|  /
                \/           \/            \/     \/    \/     \/     \/ 
    "#;
    println!("{}", ascii);
    loop {
        println!("Password manager menu:");
        println!("1. Add entry");
        println!("2. List entries");
        println!("3. Search entry");
        println!("4. Quit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clr();
                let entry = ServiceInfo::new(
                    
                )
            }
            "2" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords: {}", err);
                    Vec::new()
                });
                for item in &services {
                    println!(
                        "Service = {}
                        - Username: {}
                        - Password: {}",
                        item.service, item.username, item.password
                    )
                }
            }
            "3" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords: {}", err);
                    Vec::new();
                });
                let search = prompt("Search:");
                for item in &services {
                    if item.service.as_str() == search.as_str() {
                        println!(
                            "Service = {}
                            - Username = {}
                            - Password = {}",
                            item.service, item.username, item.password
                        )
                    }
                }
            }
            "4" => {
                clr();
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice."),
        }
        println!("\n\n");
    }
}
