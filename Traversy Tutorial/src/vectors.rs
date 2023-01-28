//Vectors -- resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    numbers[2] = 20;

    numbers.push(5);
    numbers.push(6);

    //Pop

    numbers.pop();

    println!("{:?}", numbers);

    // Get single value
    println!("Single Value: {}", numbers[0]);

    //Get vector length
    println!("Vector Length: {}", numbers.len());

    //Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    //loop

    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}