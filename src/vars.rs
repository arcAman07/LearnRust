#![allow(non_snake_case)]
// #[warn(unused_variables)]
pub fn run(){
    let name = "Aman";
    let mut age = 19;
    println!("{} is a God and his age is {}", name, age);
    age = 20;
    println!("{} is a God and his age is {}", name, age);
    // Define constant
    const ID: i32 = 001; //i32 defines type which is integer32
    println!("ID : {}",ID);
    //Assign multiple variables
    let (my_name, my_age) = ("Aman",19);
    println!("{} is {}", my_name, my_age);

}