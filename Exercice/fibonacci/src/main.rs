fn main() {
    let result = fibonnacci(6);
    println!("The result is: {result}")
}

fn fibonnacci(x: u32) -> u128 {
    match x {
        0 => 0,
        1 => 1,
        _ => {
            let mut cache: [u128; 2] = [0, 1];

            for _ in 1..x {
                let temp = cache[1];
                cache[1] = cache[0] + temp;
                cache[0] = temp;
            }
            cache[1]
        }
    }
}
