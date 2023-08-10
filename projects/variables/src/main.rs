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
}
