enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(side) => side * side,
        Shape::Rectangle(length, width) => length * width,
    }
}

fn main() {
    let circle: Shape = Shape::Circle(5.0);
    let square: Shape = Shape::Square(4.0);
    let rectangle: Shape = Shape::Rectangle(3.0, 6.0);

    println!("Circle area: {}", calculate_area(circle));
    println!("Square area: {}", calculate_area(square));
    println!("Rectangle area: {}", calculate_area(rectangle));
}
