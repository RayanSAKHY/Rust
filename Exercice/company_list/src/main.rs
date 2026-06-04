use std::collections::BTreeMap;
use std::io;

fn main() {
    println!("Welcome to the company list");

    let command_phrase = "Here are the commands:\n1: Adding a new employee\n2: Recover all the employee in a department\n3: Recover all the employee in the company\n4: Quit";

    let mut map = BTreeMap::new();
    println!("{}", command_phrase);
    loop {
        let user_input = user_interface("What do you want to do ?");

        match user_input.trim().parse::<i32>() {
            Ok(num) => {
                if num < 1 || num > 4 {
                    println!("Please enter a number between 1 and 4");
                } else {
                    action(num, &mut map);
                    println!("{}", command_phrase);
                }
            }
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        }
    }
}

fn action(choice: i32, map: &mut BTreeMap<String, Vec<String>>) {
    match choice {
        1 => {
            let name = user_interface("What is the name of the employee ?");
            let department = user_interface("In what department is he working ?");
            add_map(map, name, department);
            println!("{map:?}");
        }
        2 => {
            let department =
                user_interface("From what department do you want to obtain the list of employee ?");
            println!("{}", employee_department(department, map));
        }
        3 => {
            for department in map.keys() {
                println!("{}", employee_department(department.to_string(), map));
            }
        }
        4 => std::process::exit(1),
        _ => println!("error"),
    };
}

fn employee_department(department: String, map: &BTreeMap<String, Vec<String>>) -> String {
    let mut phrase = String::new();
    match map.get(&department) {
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
    return phrase;
}

fn add_map(map: &mut BTreeMap<String, Vec<String>>, name: String, department: String) {
    let vect = map.entry(department).or_insert(Vec::new());
    vect.push(name);
}

fn user_interface(question: &str) -> String {
    println!("{}", question);

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    return user_input.trim().to_string();
}
