use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main(){
    // let secret = rand::thread_rng().gen_range(1..=100);
    println!("The guessing game!");
   // println!("The secret number is {secret}");
    let mut name = String::new();
    println!(" what is your name?");
    io::stdin().read_line(&mut name).expect("failed");

    new_game(name);

    // println!("Input a number btw 1-100");
    // loop{
    //     let mut guess = String::new();

    //      io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed");

    //         let guess: u32 = match guess.trim().parse(){
    //             Ok(num) => num,
    //             Err(_) => continue,
    //         };

    //     println!("you guessed - {guess}");

    //     match guess.cmp(&secret){
    //         Ordering::Less => println!("Too small"),
    //         Ordering::Greater => println!("Too big"),
    //         Ordering::Equal => {
    //             println!("You win");
    //             break;
    //         }
    //     }
  //  }
}

fn new_game(name: String){
    let secret = rand::thread_rng().gen_range(1..=100);
    let mut x: u32 = 0;
    let name = name.trim();

    println!("hello {name} input a number btw 1-100");
    
    loop{
        let mut guess = String::new();

         io::stdin()
            .read_line(&mut guess)
            .expect("Failed");

      //  let guessNum = guess.trim().parse();

        let guess: u32 = match guess.trim().parse(){
                Ok(num) => num,
                Err (_) => {
                    println! ("enter valid");
                    continue;
                }
            
            };
        x = x+1;

        println!("you guessed - {guess}");

        match guess.cmp(&secret){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win with {x} tries");
                break;
            }
        }
    }    

}
