use std::sync::mpsc::channel;
use std::thread::spawn;

fn main() {
    let (tx, rx) = channel();

    for i in 0..10 {
        let producer = tx.clone();

        spawn(move || {
            let mut sum: u64 = 0;
            for j in (i * 10000000)..((i + 1) * 10000000) {
                sum += j;
            }
            producer.send(sum).unwrap();
        });
    }
    drop(tx);

    let mut total: u64 = 0;
    for val in rx {
        total += val;
    }
    println!("The sum from 0 to 1e8 - 1 is {}", total);
}
