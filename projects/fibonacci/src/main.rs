use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    let number: i32 = args[1].trim().parse().expect("Please type a number !");
    println!("Fibonacci de {} est {}", number, fibonacci(number));
}

fn fibonacci(index: i32) -> i32 {
    if index < 0 {
        return -1;
    }
    if index == 0 {
        return 0;
    }
    if index == 1 {
        return 1;
    }
    return fibonacci(index - 1) + fibonacci(index - 2)
}
