fn main() {
    let str = String::from("Bhuvansh");
    let length = get_str_len(&str);
    println!("Length of string '{}' is {}", str, length);
}

fn get_str_len(s: &String) -> usize {
    s.chars().count()
}
