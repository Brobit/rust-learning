use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
    
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

/*
fn main() {
    let mut x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // addition
    let sum = 5 + 10;

    // substracion
    let difference = 95.5 - 5.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let trucated = -5 / 3;

    // remainder
    let remainder = 43 % 5;

    // boolean
    let t = true;

    let f: bool = false;

    // char
    let c = 'z';

    let z: char = 'Z';

    let heart_eyed_cat = '😻';

    // tuple 
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // array

    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a: [3; 5];  // initialize an array of 5 full of 3
    // is the same as
    let a = [3, 3, 3, 3, 3];

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    
    let second = a[1];
}
*/
