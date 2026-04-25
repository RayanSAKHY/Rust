fn main() {
    let phrase = String::from("hello world");
    let world = slice(&phrase);

    println!("{world}");
}

fn slice(phrase: &str) -> &str {
    let bytes = phrase.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &phrase[0..i];
        }
    }

    &phrase[..]
}
