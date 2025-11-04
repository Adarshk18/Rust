fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {   //right now its taking ownership 
    if a.len() > b.len() {
        return a;
    } else {
        return b;
    }
}

//without ownership


fn main() {
    let longest_str;
    let str1 = String::from("small");
    {
        let str2 = String::from("longer");
        longest_str = longest(&str1, &str2);  //as we have used lifetimes so longest_str lifetime ends here we cant use after this .
    }
    // println!("{}", longest_str);
}
