fn five() -> i32 {
    45
}

fn main(){


    //functions that returns something
    let x = five();
    println!("The value of x is: {x}");
    
    another_function(3, 'c');

}

fn another_function(x: i32, unit_label: char){
    println!("The Value is: {x}{unit_label}");

}