fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &character) in bytes.iter().enumerate() {
        if character == b' ' {
            return &s[..i];
        }
    }

    return s;
}

fn main() {
    let mut hello = String::from("💖 Rust");

    println!("{}", hello);
    hello.push('w');
    println!("{}", hello);

    println!("{}","1----------");
    for b in hello.as_bytes() {
        println!("{}", b);
    }
    println!("{}","2----------");
    for c in hello.chars() {
        println!("{}", c);
    }
    println!("{}","3----------");

    let word = first_word(&hello);
    println!("First word: {}", word);
}
