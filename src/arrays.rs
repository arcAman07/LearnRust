#![allow(non_snake_case)]
pub fn run() {
    use std::mem;
    let mut numbers : [i32;5] = [1,2,3,4,5];
    // Print all the numbers
    println!("{:?}",numbers);
    // Get a single value
    println!("{}",numbers[1]);
    // Reassign a value in the array
    numbers[4] = 7;
    println!("{:?}",numbers);
    // Get array Length
    println!("Length of the array is {}",numbers.len());
    // Arrays are stack allocated
    println!("Array occupies {} bytes",mem::size_of_val(&numbers));
    // Get Slice
    let slice: &[i32] = &numbers[0..4];
    println!("Slice: {:?}",slice);
}