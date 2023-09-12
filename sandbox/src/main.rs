use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    game();
}

fn game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Pls input num:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("F");


        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you have guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Win win");
                break;
            }
        }
    }
}
