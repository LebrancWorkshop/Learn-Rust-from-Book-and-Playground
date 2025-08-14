fn main() {
    let friends: [&str; 3] = ["Jomina", "Yusof", "Iwan"];
    for friend in friends {
        println!("Hello, {friend}");
    }
}
