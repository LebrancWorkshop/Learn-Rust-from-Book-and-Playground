fn main() {
    let mut counter = 99;
    let num = loop {
        counter -= 1;

        if counter == 89 {
            break counter;
        }
    };
    println!("Num: {}", num);
}
