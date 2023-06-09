use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number! : {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();
    // io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess = " 0 ".to_string();
    let guess: u32 = guess.trim().parse().expect("Please type a number");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
