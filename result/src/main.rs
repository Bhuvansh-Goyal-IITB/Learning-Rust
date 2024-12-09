use std::fs::read_to_string;

fn main() {
    let file_content = read_to_string("./a.txt");
    match file_content {
        Ok(result) => println!("The contents of the file are: {}", result),
        Err(error) => println!("Error occured: {}", error),
    }
}
