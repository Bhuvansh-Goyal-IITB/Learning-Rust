fn main() {
    let name = String::from("Hello World");
    let first_word = get_first_word(&name);

    println!("Sentence is -> {}", name);
    println!("First word is -> {}", first_word);
}

fn get_first_word(str: &str) -> &str {
    let mut index = 0;

    for char in str.chars() {
        if char == ' ' {
            break;
        }
        index += 1;
    }

    return &str[0..index];
}
