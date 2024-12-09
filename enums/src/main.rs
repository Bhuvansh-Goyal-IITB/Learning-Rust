enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
}

fn main() {
    let rect = Shape::Rectangle(12.0, 6.0);
    let circle = Shape::Circle(10.0);

    match rect {
        Shape::Rectangle(w, h) => {
            println!("Area of rect (w: {}, h: {}) is {})", w, h, get_area(rect))
        }
        Shape::Circle(_) => {}
    }

    match circle {
        Shape::Rectangle(_, _) => {}
        Shape::Circle(r) => println!("Area of circle (r: {}) is {})", r, get_area(circle)),
    }
}

fn get_area(shape: Shape) -> f64 {
    match shape {
        Shape::Rectangle(w, h) => w * h,
        Shape::Circle(r) => std::f64::consts::PI * r * r,
    }
}
