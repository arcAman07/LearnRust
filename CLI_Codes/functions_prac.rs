#![allow(non_snake_case)]
use std::io;
pub fn run() {
    let mut a = String::new() ;
    let mut b = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read line");
    io::stdin().read_line(&mut b).expect("Failed to read line");
    let c:f32 = b.trim().parse::<f32>().unwrap();
    let d:f32 = a.trim().parse::<f32>().unwrap();
    let a = a.trim().parse::<i32>().unwrap();
    let b = b.trim().parse::<i32>().unwrap();
    println!("Addition of these 2 numbers is {}",add(a,b));
    println!("Subtraction of these 2 numbers is {}",subtract(a,b));
    println!("Multiplication of these 2 numbers is {}",multiply(a,b));
    println!("Division of these 2 numbers is {}",divide(c,d));
    println!("Remainder of these 2 numbers is {}",remainder(a,b));



}
pub fn add(x:i32,y:i32) -> i32 {
    x+y
}
pub fn subtract(x:i32,y:i32) -> i32 {
    x-y
}
pub fn multiply(x:i32,y:i32) -> i32 {
    x*y

}
pub fn divide(x:f32,y:f32) -> f32 {
    x/y
}
pub fn remainder(x:i32,y:i32) -> i32 {
    x%y
}
