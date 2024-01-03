use std::rc::Rc;

#[derive(Debug)]
struct Car {
    name: String,
    year: u32,
    hp: u32,
    mileage: u32,
}

impl Drop for Car {
    fn drop(&mut self) {
        println!("Called drop for vehicle: {}", self.name);
    }
}

fn box_example() {
    let car1 = Box::new(Car {
        name: String::from("Audi RS3"),
        year: 2022,
        hp: 400,
        mileage: 0,
    });

    // let car2 = car1;

    println!("box {:#?}", car1);
    // println!("{:#?}", car2);
}

fn print_type_of<T>(_: &T) {
    println!("Type: {}", std::any::type_name::<T>())
}

fn rc_example() {
    println!("1ex");
    let car1 = Rc::new(Car {
        name: String::from("Audi RS4"),
        year: 2022,
        hp: 400,
        mileage: 0,
    });
    print_type_of(&car1);
    println!("2ex");

    {
        let car2 = car1.clone();
        println!("{:#?}", car2);
        print_type_of(&car2);
    }
    println!("3ex");
    println!("{:#?}", car1);
}

fn main() {
    println!("1main");
    box_example();
    rc_example();
    println!("2main");
}
