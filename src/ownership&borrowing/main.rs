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
    // let s = String::from("hello");

    // takes_ownership(s);

    // let x= 4;
    // makes_copy(x);

    // let s1 = gives_ownership();

    // let s2 = String::from("hello");
    // let s3 = takes_and_gives_back(s2);

    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);

    // println!("The length of string {s1} is {len}");

    let mut s1 = String::from("hello");
    change(&mut s1);

    let reference_to_nothing = dangle();


}

// fn calculate_length(s: &String) -> usize{
//     s.len()
// }

fn change(some_string: &mut String){
    some_string.push_str(", world");
}

fn dangle() -> &String{
    let s = String::from("hello");

    &s
}

// fn takes_ownership(some_string: String){
//     println!("{some_string}");
// }

// fn makes_copy(some_integer: i32){
//     println!("{some_integer}")
// }

// fn gives_ownership() -> String{
//     let some_string = String::from("yours");
//     some_string
// }


// fn takes_and_gives_back(a_string: String) -> String{
//     a_string
// }

