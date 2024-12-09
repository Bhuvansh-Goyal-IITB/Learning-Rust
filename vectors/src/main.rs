fn main() {
    //let mut vector = Vec::new();
    //vector.push(1);
    //vector.push(2);
    //vector.push(4);
    //vector.push(3);

    let vector = vec![1, 2, 3, 5];

    let filtered_vector = even_filter(&vector);

    println!("The original elements are: {:?}", vector);
    println!("The even elements are: {:?}", filtered_vector);
}

fn even_filter(vector: &Vec<i32>) -> Vec<i32> {
    let mut filtered_vector = Vec::new();
    for val in vector {
        if val & 1 == 0 {
            filtered_vector.push(*val);
        }
    }

    return filtered_vector;
}
