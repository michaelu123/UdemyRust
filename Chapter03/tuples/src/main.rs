// Tuples: Fixed length, different dtypes possible

fn main() {
    let mut tpl = (500, "hi", true);

    println!("{:?}", tpl);

    println!("First: ");
    println!("{}", tpl.0);
    println!("{}", tpl.1);
    println!("{}\n", tpl.2);

    let (x, y, z) = tpl;

    println!("Second: ");
    println!("x{}", x);
    println!("y{}", y);
    println!("z{}\n", z);

    tpl.0 = 400;

    let (x, y, z) = tpl;
    println!("Third: ");
    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}\n", z);
}
