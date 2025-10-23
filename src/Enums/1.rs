// enum Direction{
//     North,
//     East,
//     South,
//     West,
// }

// fn main(){
//     let my_direction: Direction = Direction::North;

//     move_direction(my_direction);
// }

// fn move_direction(direction: Direction){

// }

enum Shape{
    Circle(f64),
    Square(f64),
    Rectangle(f64,f64),
}

fn calculate_area(shape: Shape) -> f64{
    return 0;
}

fn main(){
    let circle: Shape = Shape::Circle(5.0);
    let square: Shape = Shape::Square(4.0);
    let rectangle: Shape = Shape::Rectangle(3.0,6.0);

    calculate_area(circle);
    calculate_area(square); 
}