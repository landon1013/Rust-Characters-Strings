// Strings
fn strings() {
    let s:&'static str = "Hello there!";

    for c in s.chars().rev() { //Can be manipulated like any other array
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("First letter is {}", first_char);
    }

    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }

    println!("{}", letters);
}

fn main() {
    strings();
}
