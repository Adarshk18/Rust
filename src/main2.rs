// fn five() -> i32 {
//     45
// }

fn main(){


    //functions that returns something
    // let x = five();
    // println!("The value of x is: {x}");
    
    // another_function(3, 'c');

    /*Branches */

    // let number = 5;

    // if number <5 {
    //     println!("conditionwas true");
    // }else{
    //     println!("condition was false");
    // }

    // let mut counter = 0;

    // let result = loop{
    //     counter +=1;

    //     if counter==10{
    //         break counter*2;
    //     }
    // };
    // println!("The result is {result}");

    /*Loop Labels */
    let mut count = 0;
    'counting_up: loop{
        println!("count = {count}");
        let mut remaining = 10;

        loop{
            println!("remaining = {remaining}");
            if remaining == 9{
                break;
            }

            if count == 2{
                break 'counting_up;
            }
            remaining -=1;
        }
        count +=1;
    }
    println!("End count = {count}");
}

// fn another_function(x: i32, unit_label: char){
//     println!("The Value is: {x}{unit_label}");

// }