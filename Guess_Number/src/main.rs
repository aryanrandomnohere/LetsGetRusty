use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number !");
    let scret_number = rand::thread_rng().gen_range(1,100);
    println!("Secret number is: {}", scret_number);
    loop{
    println!("Please input your guess");
    let mut guess = String::new();   
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess:u32 = guess.trim().parse().expect("Are you sure that you entered a number");
    println!("You guessed: {}", guess);
    match guess.cmp(&scret_number) {
        Ordering::Less => {
                println!("Number Too low, Please try again ");
                continue;
            },
        Ordering::Greater => {
                println!("Number too high,Please try again");
                continue;
            },
        Ordering::Equal => {
                println!("You win!,Game Ends");
                break;
            },
    }
    }
}
