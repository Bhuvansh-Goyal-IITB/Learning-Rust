struct User {
    name: String,
    age: u32,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn debug() {
        println!("This is similar to a static function")
    }
}

fn main() {
    let user = User {
        name: String::from("Bhuvansh"),
        age: 21,
    };

    let rect = Rectangle {
        width: 12.0,
        height: 6.0,
    };

    println!("user.name -> {}", user.name);
    println!("user.age -> {}", user.age);

    println!(
        "Area of rect (w: {}, h: {}) is {}",
        rect.width,
        rect.height,
        rect.area()
    );
    println!(
        "Perimeter of rect (w: {}, h: {}) is {}",
        rect.width,
        rect.height,
        rect.perimeter()
    );

    Rectangle::debug();
}
