use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess the number!");
    print_rules();

    let secret_number = generate_secret_number(1, 101);

    loop {
        let input_number = ask_number();
        if compare_number(input_number, secret_number) {
            break;
        }
    }
}

fn print_rules() {
    /* Function which prints the rules */

    println!("The rules are simple, you have to guess the random number generated");
}

fn ask_number() -> i32 {
    /* Function which asks for a number on standard input */
    let mut number = String::new();

    println!("Tell me a number:");
    io::stdin().read_line(&mut number).expect("Error with input");

    let number: i32 = number.trim().parse().expect("Error on parse");

    println!("Input number: {}", number);

    return number;
}

fn generate_secret_number(min: i32, max: i32) -> i32 {
    let secret_number = rand::thread_rng().gen_range(min..max);

    return secret_number;
}

fn compare_number(input_number: i32, secret_number: i32) -> bool {
    match input_number.cmp(&secret_number) {
        Ordering::Less => {
            println!("Secret number is greater");
            return false;
        }
        Ordering::Greater => {
            println!("Secret number is lower");
            return false;
        }
        Ordering::Equal => {
            println!("Congrats! You found it!");
            return true;
        }
    }
}