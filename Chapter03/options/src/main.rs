// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    let x: u32 = 5;
    let y: Option<u32> = Some(6);

    println!("x + y = {}", addu(x, y));
    let a = -1;
    let b = Some(4);
    println!("(i32) a + b = {}", addi(a,b));

    let c = -2;
    let d = Some(5);
    println!("(i8) c + d = {}", addi8(c,d));
}

fn addu(x: u32, y: Option<u32>) -> u32 {
    match y {
        Some(val) => x + val,
        None => x,
    }
}

fn addi(x: i32, y: Option<i32>) -> i32 {
    match y {
        Some(val) => x + val,
        None => x,
    }
}

fn addi8(x: i8, y: Option<i8>) -> i8 {
    match y {
        Some(y) => x + y,
        None => x,
    }
}
