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

// Format
fn format() {
    let name = "Dmitri";
    let greeting = format!("Hi, I'm {}, nice to meet you", name);
    println!("{}", greeting);

    let hello = "Hello";
    let rust = "Rust";
    let hello_rust = format!("{}, {}!", hello, rust);
    println!("{}", hello_rust);

    let run = "Run";
    let forest = "Forest";
    let rfr = format!("{0}, {1}, {0}", run, forest);
    println!("{}", rfr);

    let info = format!(
        "The name's {last}. {first} {last}.",
        first = "James",
        last = "Bond"
    );
    println!("{}", info);

    let mixed = format!(
        "{1} {} {0} {} {data}",
        "alpha",
        "beta",
        data = {"delta"}
    );
    println!("{}", mixed);
}

fn main() {
    //strings();
    format();
}
