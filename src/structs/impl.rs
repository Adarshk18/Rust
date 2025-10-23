struct Rect{
    width: u32,
    height: u32,
}

// impl Rect{
//     fn area(&self) -> u32{
//         return self.widht * self.height
//     }
//     fn perimeter(&self) -> u32{
//         return 2*(self.width + self.height);
//     }
// }

/*using traits */
impl Debug for Rect{
    fn format(&self, f: &mut Formatter) -> fmt::Result{
        write!(f,"Rect {{ width: {}, height: {} }}",self.width,self.height)
    }
}

fn main(){
    let rect: React = React{
        width: 30,
        height: 50,
    };
    println!("The area of rectangle ios {}",rect.area());
    println!("{:?}",rect);
}