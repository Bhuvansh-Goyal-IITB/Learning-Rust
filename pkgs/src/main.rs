use chrono::{Local, Utc};

fn main() {
    let now = Local::now();
    println!("The local time now is {}", now);

    let utc_now = Utc::now();
    println!("The utc time now is {}", utc_now);
}
