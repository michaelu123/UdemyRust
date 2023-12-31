fn main() {
    let number = 111;

    match number {
        1 => {
            println!("One!");
            println!("One!");
        },
        2 | 3 | 5 | 7 | 11 => println!("Prime!"),
        _ => println!("Default!"),
    }

    let boolean = true;

    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);

    let s = match "f" {
        "a" => "A",
        "b" => "B",
        "c" => "C",
        "d"|"e" => "DE",
        _ => "?",
    };
    println!("s: {}", s);
}
