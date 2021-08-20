#![allow(non_snake_case)]
pub fn run() {
    let person: (&str,&str,i32) = ("Aman","Mumbai",19);
    println!("{} is from {} and {} years old",person.0,person.1,person.2);
}