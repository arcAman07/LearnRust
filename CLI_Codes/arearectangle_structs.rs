#![allow(non_snake_case)]
use std::io;
struct Rectangle {
        length : i32,
        breadth : i32

    }
pub fn run() {
    // let mut _number = String::new();
    // io::stdin().read_line(&mut _number).expect("Failed to read line");
    // let mut c:i32 = _number.trim().parse::<i32>().unwrap();
    // loop {
    //     println!("I am A GOD");
    //     c = c+1;
    //     if c==10 {
    //         break;
    //     }

    // }

    let mut a1 = String::new();
    let mut a2 = String::new();
    io::stdin().read_line(&mut a1).expect("Failed to read line");
    io::stdin().read_line(&mut a2).expect("Failed to read line");
    let a1 = a1.trim().parse::<i32>().unwrap();
    let a2 = a2.trim().parse::<i32>().unwrap();
    let rect1 = Rectangle { 
        length : a1,
        breadth : a2
    };
    println!("Area of the rectangle is {}",area(&rect1));
    }
    fn area(rectangle: &Rectangle) -> i32 {
        rectangle.length * rectangle.breadth
    }
//     struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }

    