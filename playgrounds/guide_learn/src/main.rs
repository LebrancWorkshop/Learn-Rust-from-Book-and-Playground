fn calculate_tax(price: f64) -> f64 {
    return price * 0.07;
}

fn main() {
    let tax = calculate_tax(98.00);
    println!("Tax: {}", tax);
}
