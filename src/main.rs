use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Угадай число!");
    println!("Введи свое число!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("WTF");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Твое число: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Меньше!"),
            Ordering::Greater => println!("Больше!"),
            Ordering::Equal => {
                println!("С победкой!");
                break;
            }
        }
    }
}
