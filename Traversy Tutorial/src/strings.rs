pub fn run() {
    let mut hello = String::from("Hello");

    //Get length

    println!("Length: {}", hello.len());

    hello.push('W');

    hello.push_str("orld!");

        
    println!("Capacity: {}", hello.capacity());

    println!("Is Empty: {}", hello.is_empty());

    println!("{}", hello);

    //Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //Assertion testing
    assert_eq!(3, s.len());

    println!("{}", s);
}