// Arrays: Fixed length, only same dtypes

fn main() {
    let _array = [1, 2, 3,4,5];

    println!("{:?}", _array);
    println!("{:#?}", _array);
    println!("{}", _array[0]);
    println!("{}", _array[1]);
    println!("{}", _array[2]);

    let _array2: [u32; 5] = [1337; 5];

    println!("{:?}", _array2);
    println!("{}", _array2[0]);
    println!("{}", _array2[1]);
    println!("{}", _array2[2]);

    let n=_array[4];
    println!("{}", _array2[n]);
}
