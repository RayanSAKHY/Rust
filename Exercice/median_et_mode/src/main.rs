use std::collections::HashMap;
use std::io;

fn main() {
    println!("Median et Mode");

    let mut v = create_vect();
    println!("Vect {v:?}");

    let median: f32 = calculate_median(&mut v);
    println!("Median equal {median}");

    let mode: f32 = calculate_mode(&mut v);
    println!("Mode equal {mode}");
}

fn calculate_mode(vect: &mut Vec<i32>) -> f32 {
    let mut map = HashMap::new();

    let mut maxi: i32 = 0;
    let mut result: f32 = *vect.get(0).unwrap() as f32;

    for number in vect.iter() {
        let count = map.entry(number).or_insert(0);
        *count += 1;

        if count > &mut maxi {
            maxi = *count;
            result = *number as f32;
        }
    }

    return result;
}

fn calculate_median(vect: &mut Vec<i32>) -> f32 {
    vect.sort();

    let len_vect = vect.len();

    let median: f32;
    if len_vect % 2 == 0 {
        median = ((*vect.get(len_vect / 2 - 1).unwrap() + *vect.get(len_vect / 2).unwrap()) as f32)
            / 2.0;
    } else {
        median = *vect.get(len_vect / 2).unwrap() as f32;
    }

    return median;
}

fn create_vect() -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();

    loop {
        println!("Do you want to add a number (Y or N) :");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        if choice.trim().to_lowercase().eq("y") {
            let value = number_chooser();
            v.push(value);
            println!("You added: {value}");
        } else {
            if v.is_empty() {
                for i in 1..5 {
                    v.push(i);
                }
            }

            break;
        }
    }

    return v;
}

fn number_chooser() -> i32 {
    let number: i32;

    println!("Enter the number you want to add");

    loop {
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse() {
            Ok(num) => {
                number = num;
                break;
            }
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };
    }

    return number;
}
