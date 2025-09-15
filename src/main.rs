use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main(){
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("The guessing game!");
    println!("The secret number is {secret}");
    println!("Input a number btw 1-100");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed");

    let guess: u32 = guess.trim().parse().expect("Please type a number");
    println! ("you guessed - {guess}");

    match guess.cmp(&secret){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win"),
    }

}

