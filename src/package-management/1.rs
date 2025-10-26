use chrono::{Local,Utc};

fn main(){
    let now = Local::now();
    println!("current time is: {}",now);
}