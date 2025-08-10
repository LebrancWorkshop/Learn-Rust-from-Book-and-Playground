use std::io;

fn main() {
    println!("Guess the number: ");
    let mut inp = String::new();

    io::stdin().read_line(&mut inp).expect("Must be the number");

    println!("Number: {}", inp);
}
