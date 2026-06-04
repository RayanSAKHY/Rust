use std::io;

fn main() {
    println!("Which word do you want to translate in Pig Latin");

    let mut sentence = String::new();

    io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to read line");

    let list_vowel = ['a', 'e', 'i', 'y', 'o', 'u'];

    let mut result = String::new();
    for word in sentence.split_whitespace() {
        let mut s2 = String::new();
        let mut s3 = String::new();

        let mut chars = word.chars();

        let mut c: char = chars.next().unwrap();

        while !list_vowel.contains(&c.to_ascii_lowercase()) {
            s2.push(c);
            c = chars.next().unwrap();
        }
        let rest: String = chars.collect();
        if s2.is_empty() {
            s3.push('h');
        }

        s3.push_str("ay");
        result.push_str(format!("{rest}{s2}{s3} ").trim());
        result.push(' ');
    }

    println!("{result}");
}
