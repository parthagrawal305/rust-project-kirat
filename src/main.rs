fn main() {
    let sentence = String::from("My name is Parth.");
    let first_word = get_first_word(sentence);
    // for i in 0..10000000 {
    //     println!("Hello, World!{}", i);
    // }
    println!("First word is: {}", first_word);
}

fn get_first_word(sentence: String) -> String {
    let mut ans = String::from("");
    for char in sentence.chars() {
        if char == ' ' {
            break;
        }
        ans.push_str(char.to_string().as_str());
    }
    return ans;
}
