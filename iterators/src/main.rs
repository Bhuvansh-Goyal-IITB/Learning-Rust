fn main() {
    let v1 = vec![1, 2, 5];

    println!("--------- Normal iterator ---------");

    let v1_iter = v1.iter();

    let mut i = 0;
    for num in v1_iter {
        println!("[{}] -> {}", i, num);
        i += 1;
    }

    println!("--------- Mutable iterator ---------");

    let mut v2 = vec![1, 3, 4];
    println!("Original vec {:?}", v2);

    let v2_iter = v2.iter_mut();

    // This is a simpler way to do this
    //for num in v2_iter {
    //    *num += 1;
    //}

    // This is another way to do this
    let mut m_iter = v2_iter;
    while let Some(num) = m_iter.next() {
        *num += 1
    }

    println!("Modified vec {:?}", v2);

    println!("--------- Consuming functions ---------");

    let v3 = vec![1, 2, 3];

    let v3_iter = v3.iter();

    // This consumes the iterator so v3_iter cannot be used after this
    let sum: i32 = v3_iter.sum();
    println!("Sum of the array {:?} is {}", v3, sum);

    println!("--------- Iterator Adapters ---------");

    let v4 = vec![1, 2, 3];

    let iter1 = v4.iter();
    let iter2 = v4.iter().map(|x| x + 1);
    let iter3 = v4.iter().filter(|x| *x % 2 == 0);

    for (i, num) in iter1.enumerate() {
        println!("iter1[{}] -> {}", i, num);
    }
    println!("------------------------");
    for (i, num) in iter2.enumerate() {
        println!("iter2[{}] -> {}", i, num);
    }
    println!("------------------------");
    for (i, num) in iter3.enumerate() {
        println!("iter3[{}] -> {}", i, num);
    }

    println!("--------- Iterator -> Vector ---------");

    let v5 = vec![1, 2, 5];
    let v6 = filter_and_map(&v5);

    println!("Original -> {:?}", v5);
    println!("Modified -> {:?}", v6);

    // Iterators for hashmaps are very similar, they just return a tuple (key, value) when
    // iterating over them
}

fn filter_and_map(vector: &Vec<i32>) -> Vec<i32> {
    let iter = vector.iter().filter(|x| *x & 1 == 1).map(|x| x * 2);
    return iter.collect();
}
