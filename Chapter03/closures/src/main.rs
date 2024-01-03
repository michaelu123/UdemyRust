fn my_function(inp: i32) -> i32 {
    inp * 2
}

fn print_type_of<T>(_: &T) {
    println!("Type: {}", std::any::type_name::<T>())
}

fn my_function2() {
    let num = 2;

    let cl1 = |inp: i32| -> i32 { inp * 2 };
    let cl2 = |inp: i32| -> i32 { inp * 3 };
    print_type_of(&cl1);
    print_type_of(&cl2);
    let ret2 = cl1(num);
    println!("a{}", ret2);

    let ret3 = cl2(num);
    println!("b{}", ret3);
}

fn main() {
    let num = 2;

    let ret = my_function(num);
    println!("c{}", ret);

    my_function2();
}
