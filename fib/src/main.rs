fn main() {
    let index = 3;
    println!("fibonacci number at index {} is {}", index, fib(index));
}

fn fib(index: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;

    if index == 0 {
        return first;
    }

    if index == 1 {
        return second;
    }

    for _ in 0..(index - 1) {
        let temp = second;
        second += first;
        first = temp;
    }

    return second;
}
