fn main() {
    let score = 60;
    let is_ok = if score >= 50 { String::from("OK") } else { String::from("Nah") };


    println!("Is OK?: {is_ok}");

}
