use std::fs;

//option enum is introduced to handle the concept of nullability in safe way.

fn find_first(s: String) -> Option<i32>{
    for (index,character) in s.chars().enumerate(){
        if character == 'a'{
            return Some(index as i32);
        }
    }
}


fn main(){
    let res = fs::read_to_string("example.txt");

    // match res{
    //     Ok(content) => {
    //         println!("File content: {}", content);
    //     },
    //     Err(err)=>{
    //         println!("Error: {}",err);
    //     }
    // }
    // println!("Hi there");


    // if let Ok(content) = res{
    //     return Ok(content);
    // }else{
    //     return Err("Error reading file".to_string());
    // }

    let my_string = String::from("random");
    match find_first(my_string){
        Some(index) => println!("The letter 'a' is found at index: {}",index),
        None => println!("The letter 'a' is not found in the string"),
    }
}