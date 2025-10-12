// fn main(){
//     let width1 = 30;
//     let height1 = 50;

//     println!("The area of rectangle is {} square pixels.", area(width1,height1));


// }

// fn area(width: u32, height: u32) -> u32{
//     width*height
// }

//using tuples

// fn main(){
//     let rect1 = (30,50);

//     println!("The area of reactangle is {} square pixels.", area(rect1));
// }

// fn area(dimensions: (u32,u32)) -> u32{
//     dimensions.0*dimensions.1
// }

/*some other way adding more meaning */

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

fn main(){
    let scale = 2;
    let rect1 = Rectangle{
        width: debug!(30*scale),
        height: 50,
    };

    // println!("The area of rectangle is {} square pixels.", area(&rect1));
    // println!("rect1 is {rect1:?}");

    dbg!(&rect1);
}

// fn area(rectangle: &Rectangle) -> u32{
//     rectangle.widht*rectangle.height
// }