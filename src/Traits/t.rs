//similar to abstract class in java & similar to interfaces in javascipt

trait Summary{
    fn summarise(&self) -> String;
}

trait Fix{
    fn fix(&self) -> String{
        return String::from("This is a fix trait");
    }
}

struct User{
    name: String,
    age: u32,
}

struct Fix;

impl Summary for User{
    fn summarise(&self) -> String{
        return format!("The name is {}, and the age is {}",self.name, self.age);
    }
}



impl Summary for Fix{}
impl Summary for String{}
impl Fix for User{}

fn main(){
    let user = User{
        name: String::from("Adarsh"),
        age: 24,
    };
   
    notify(user);
}

//traits as parameters
fn notify(u: impl Summary){
    println!("{}".u.summarise());
}

//Trait bound syntax,
fn notiy<T: Summary + Fix>(u: T){
    println!("{}",user.summarise());
}
