fn main() {
    let decimal_flt = 256.4321_f32;

    // let decimal_int: u8 = decimal_flt;
    let decimal_u8 = decimal_flt as u8;
    println!("Casting: {} -> {}", decimal_flt, decimal_u8);

    let decimal_u16 = decimal_flt as u16;
    println!("Casting: {} -> {}", decimal_flt, decimal_u16);

    let value = 129;
    let decimal_i8 = value as i8;
    println!("Casting: {} -> {}", value, decimal_i8);

    let decimal_i16 = value as i16;
    println!("Casting: {} -> {}", value, decimal_i16);

    let value = -129;
    let decimal_i8 = value as i8;
    println!("Casting: {} -> {}", value, decimal_i8);

    let decimal_i16 = value as i16;
    println!("Casting: {} -> {}", value, decimal_i16);

}
