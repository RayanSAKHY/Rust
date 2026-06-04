use std::collections::BTreeMap;
use std::io;

fn main() {
    println!("Welcome to the company list");

    let command_phrase = "Here are the commands:\nAdd [Employee name] to [department name]\nList [department name]\nList All\nQuit";

    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
    println!("{}", command_phrase);
    loop {
        user_interface("What do you want to do ?", &mut map);
    }
}

fn employee_department(department: &String, map: &BTreeMap<String, Vec<String>>) -> String {
    let mut phrase = String::new();
    match map.get(department) {
        Some(vect) => {
            phrase.push_str("Department: ");
            phrase.push_str(department.as_str());
            phrase.push_str("\nEmployees:");
            for name in vect {
                phrase.push(' ');
                phrase.push_str(name);
            }
        }
        None => {
            phrase.push_str("This department don't exist");
        }
    };
    phrase
}

fn add_map(map: &mut BTreeMap<String, Vec<String>>, name: String, department: String) {
    let vect = map.entry(department).or_insert(Vec::new());
    vect.push(name);
    vect.sort();
}

fn user_interface(question: &str, map: &mut BTreeMap<String, Vec<String>>) {
    println!("{}", question);

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let user_input = user_input.to_lowercase();

    let mut words = user_input.split_whitespace();

    let mut word = words.next().expect("Wrong Input");

    match word {
        "add" => {
            let name = words.next().expect("Wrong Input");
            word = words.next().expect("Wrong Input");
            if !word.eq("to") {
                println!("Wrong Input");
                std::process::exit(1)
            }
            let department = words.next().expect("Wrong Input");
            add_map(map, name.to_string(), department.to_string());
        }
        "list" => {
            word = words.next().expect("Wrong Input");
            if word.eq("all") {
                for department in map.keys() {
                    println!("{}", employee_department(department, map));
                }
            } else {
                println!("{}", employee_department(&word.to_string(), map));
            }
        }
        "quit" => std::process::exit(0),
        _ => {
            println!("Wrong Input");
            std::process::exit(1);
        }
    };
}
