pub fn run() {
    let x = 1;

    //Default is f64
    let y = 2.5;

    //Add explicit type
    let z: i64 = 4545454545;

    //Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i32: {}", std::i64::MAX);

    //Boolean
    let is_active = true;
    let is_greater: bool = 10 > 5;

    let a1 = 'a';

    println!("{:?}", (x, y, z, is_active, is_greater, a1));
}