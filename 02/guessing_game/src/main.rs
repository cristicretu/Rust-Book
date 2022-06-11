use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Input:");

    let answer = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let number: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match number.cmp(&answer) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
