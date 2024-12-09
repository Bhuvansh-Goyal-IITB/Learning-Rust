use std::collections::HashMap;

fn group_values(vector: &Vec<(String, u32)>) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    for (key, value) in vector {
        map.insert(key.clone(), *value);
    }
    return map;
}

fn main() {
    let input = vec![(String::from("Bhuvansh"), 21), (String::from("Jaya"), 50)];
    let output = group_values(&input);

    println!("Input is -> {:?}", input);
    println!("Output is -> {:?}", output);
}
