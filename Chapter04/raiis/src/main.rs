// RAII (Ressource Acquisition Is Initialization)

#[derive(Debug)]
struct Data {
    val: i32,
}

impl Drop for Data {
    fn drop(&mut self) {
        println!("Drop is being called...");
    }
}

fn print_type_of<T>(_: &T) {
    println!("Type: {}", std::any::type_name::<T>())
}

fn main() {
    let mut box1 = Box::new(5);
    println!("box1 {}", box1);
    print_type_of(&box1);

    let d = Box::new(Data { val: 11 });
    println!("d {:?}", d);
    print_type_of(&d);
}
