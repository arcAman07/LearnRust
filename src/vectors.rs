#![allow(non_snake_case)]
pub fn run() {
    use std::mem;
    let mut numbers : Vec<i32> = vec![1,2,3,4,5];
    // Print all the numbers
    println!("{:?}",numbers);
    // Get a single value
    println!("{}",numbers[1]);
    // Reassign a value in the array
    numbers[4] = 7;
    println!("{:?}",numbers);
    // Add on to vector
    numbers.push(9);
    numbers.push(27);
    println!("{:?}",numbers);
    //Pop of the last value
    numbers.pop();
    println!("{:?}",numbers);
    // Get array Length
    println!("Length of the Vector is {}",numbers.len());
    // Arrays are stack allocated
    println!("Vector occupies {} bytes",mem::size_of_val(&numbers));
    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}",slice);
    //Loop through vector values
    for x in numbers.iter() {
        println!("Number : {}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec :{:?}",numbers);
}