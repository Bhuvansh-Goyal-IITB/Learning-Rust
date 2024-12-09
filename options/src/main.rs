fn main() {
    let string_1 = String::from("Preet");
    let mut first_a_index = get_first_a_index(&string_1);

    match first_a_index {
        Some(i) => println!("The index of the first a in {} is {}", string_1, i),
        None => println!("a not found in {}", string_1),
    }

    let string_2 = String::from("Bhuvansh");
    first_a_index = get_first_a_index(&string_2);

    match first_a_index {
        Some(i) => println!("The index of the first a in {} is {}", string_2, i),
        None => println!("a not found in {}", string_2),
    }
}

fn get_first_a_index(s: &String) -> Option<u32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as u32);
        }
    }

    return None;
}
