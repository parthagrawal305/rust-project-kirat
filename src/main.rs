fn main() {
    let greeting: String = String::from("hello world");
    println!("{}", greeting);

    let n: usize = 13;
    let char1 = greeting.chars().nth(n);

    match char1 {
        Some(c) => println!("The character at {}th postition is: {}", n, c),
        None => println!("No character found at index {}", n),
    }
}
