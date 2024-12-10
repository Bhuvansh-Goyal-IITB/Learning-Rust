use std::thread;

fn main() {
    let t1 = thread::spawn(|| {
        for _ in 0..500 {
            println!("Hello from spawned thread!");
        }
    });

    for _ in 0..500 {
        println!("Hello from main thread!");
    }

    t1.join().unwrap();
}
