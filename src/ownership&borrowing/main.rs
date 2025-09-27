fn main() {
    // let mut s = String::from("hello");
    // println!("{}", s);

    // s.push_str(", world!");
    // println!("{s}");

    // let mut s = String::from("hello");
    // s= String::from("adarsh");
    // println!("{s}");

    // let s = String::from("hello");
    // let s1 = s.clone();
    // println!("s = {s}, s1 = {s1}");

    /*ownership function */
    let s = String::from("hello");

    takes_ownership(s);

    let x= 4;
    makes_copy(x);
    

}

fn takes_ownership(some_string: String){
    println!("{some_string}");
}

fn makes_copy(some_integer: i32){
    println!("{some_integer}")
}
