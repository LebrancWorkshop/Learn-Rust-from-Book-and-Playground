fn main() {
    let mut s = String::from("String");
    let len = string_len(&s);
    waho(&mut s);
    println!("Length of {s} is {len}");
    println!("The New Sentence: {:?}.", s);

    waho(&mut s);
}

fn string_len(word: &String) -> u32 {
    word.len() as u32
}

fn waho(word: &mut String) {
    word.push_str(" waho.");
}
