use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    let number: f32 = args[1].trim().parse().expect("Please type a number !");

    match args[2].as_str() {
        "Celsius" => celsius(&number),
        "Fahrenheit" => fahr(&number),
        _ => println!("Wrong converstion, please type Celsius or Fahrenheit !"),
    }
}

fn celsius(number: &f32) {
    println!("We are going to convert {} degree Farheheit in Celsius.", number);
    //put number in celsius
    let num = (number - 32.0) * 5.0/9.0;
    println!("It makes {} degree Celsius !", num);
}

fn fahr(number: &f32) {
    println!("We are going to convert {} degree Celsius in Fahrenheit.", number);
    //put number in fahr
    let num = number * 9.0/5.0 + 32.0;
    println!("It makes {} in Fahrenheit !", num);
}
