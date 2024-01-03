// every reference has a life time
// life time annotations do not modify life time
// 'a: lifetime (spoken: tick a)

fn main() {
    // let r: &str;

    // {
    //     let string1 = String::from("test");
    //     r = &string1;
    // }

    // println!("{}", r);

    let result1: &String;
    let string1 = String::from("test11");
    let string2 = String::from("test2");
    {
        let string3 = String::from("test33");
        result1 = longest_str(&string1, &string3);
        println!("res1a {}", result1);
    }
    // println!("res1b {}", result1); // string3 does not live long enough

    let result2 = longest_str(&string1, &string2);
    println!("res2 {}", result2);
}

fn longest_str<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
