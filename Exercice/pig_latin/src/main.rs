use std::io;

fn main() {
    println!("Which word do you want to translate in Pig Latin");

    let mut word = String::new();

    loop {
        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");

        match word.trim().parse::<i32>() {
            Ok(_num) => {
                println!("Please enter a world");
                continue;
            }
            Err(_) => break,
        };
    }

    let list_vowel = ['a', 'e', 'i', 'y', 'o', 'u'];

    let mut s2 = String::new();
    let mut s3 = String::new();

    let mut c: char = word.chars().next().unwrap();

    while !list_vowel.contains(&c) {
        s2.push(c);
        word.remove(0);
        c = word.chars().next().unwrap();
    }
    if s2.is_empty() {
        s3.push('h');
    }

    s3.push_str("ay");

    let result = word.trim().to_string() + &s2 + &s3;
    println!("{result}");
}
