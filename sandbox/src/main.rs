use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    //game();
    //shadow();
    //tupll();
    nex();
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

fn shadow() {
    let x = 5;

    let x = x * x;

    {
        let x = x * 2;
        println!("Inner {x}")
    }
    println!("Outer {x}")
}

fn tupll() {
    let y = (0, 1.4, 2, 3);
    let x = y.3;

    let z = [1, 2, 3, 4];

    let t = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    const MONTHS: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a = [3; 5];

    let mut index = String::new();

    let c = {
        let c = 3;
        c * c
    };

    println!("{}", c);

    io::stdin().read_line(&mut index).expect("F");

    let index: usize = index.trim().parse().expect("L");

    //let b = a[index];

    println!("{:?}", a)
}

fn nex() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 3;
        }
    };

    println!("{result}");

    let mut count = 0;

    'up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9{
                break;
            }
            if count == 2 {
                break 'up;
            }
            remaining -=1;
        }
        count += 1;
    }
    println!("End count= {count}");


    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");


    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for nu in (1..4).rev(){
        println!("{nu}!");
    }
}
