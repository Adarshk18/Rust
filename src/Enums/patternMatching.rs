enum Shape{
    Circle(f64),
    Square(f64),
    Rectangle(f64,f64),
}

fn calculate_area(shape: Shape) -> f64{
    let ans = match  shape{
        Shape::Circle(radius) =>  3.14*radius*radius,
        Shape::Square(side_length) => side_length*side_length,
        Shape::Rectangle(width,height) => width*height,
    };

    return ans;
}

fn main(){
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0,6.0);

    let area = calculate_area(square);
    print!("Area of circle: {}\n",area);
}