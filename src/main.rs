use rand::{Rng, rng};
use std::cmp::Ordering;
use std::io::{Write, stdin, stdout};

fn main() {
    println!("\n");

    println!("Guessing number game is started ...");

    let secret_number: u32 = rng().random_range(0..=100);

    loop {
        print!("enter you guess number: ");
        stdout().flush().unwrap();

        let mut guess_number = String::new();
        stdin()
            .read_line(&mut guess_number)
            .expect("error in read data from terminal ...");

        if guess_number.trim().eq_ignore_ascii_case("quit") {
            println!("Goodbye ...");
            break;
        }

        let guess_number: u32 = match guess_number.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Your guess number is too less ..."),
            Ordering::Greater => println!("Your guess number is too big ..."),
            Ordering::Equal => {
                println!("Well Doen! You win ...");
                break;
            }
        }
    }

    println!("\n The End ... (0.0.1)\n");
}
