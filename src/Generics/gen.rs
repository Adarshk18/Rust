fn main(){
    let bigger = largest(1,2);
    let biggest = largest('a','b');
    println!("{}",bigger);
    println!("{}",biggest);
}

fn largesti32(a: i32, b: i32) -> i32{
    if a>b{
        a
    }else{
        b
    }
}

fn largestchar(a: char, b: char) -> char{
    if a>b{
        a
    }else{
        b
    }
}

//now combining the above two functions using generics
fn largest<T: std::cmp::PartialOrd>(a: t, b: T) -> T{
    if a>b{
        a
    }else{
        b
    }
}