use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn set_dificult(difficult: &mut u32, number: u32) {
    *difficult = number;
}

fn main() {
    println!("Guess the number!");

    let mut input = String::new();

    println!(
        "Please choose the difficult\n1. Easy => 10 numbers\n2. Medium => 50 numbers\n3. Hard => 100 numbers"
    );
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = match input.trim().parse() {
        Ok(abobora) => abobora,
        Err(_) => {
            println!("Invalid Coice!");
            return;
        }
    };

    let mut difficult: u32 = 0;

    match input {
        1 => set_dificult(&mut difficult, 10),
        2 => set_dificult(&mut difficult, 50),
        3 => set_dificult(&mut difficult, 100),
        _ => {
            println!("Invalid difficult!");
            return;
        }
    }

    let secret_number = rand::thread_rng().gen_range(1..=difficult);

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
