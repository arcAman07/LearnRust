#![allow(non_snake_case)]

pub fn run() {
    //Print to console
    println!("Hello from the print.rs file");
    //Basic formatting
    println!("{} is a god","Aman");
    //Postion formatting
    println!("{0} is a god {1} {0} loves to play {2}","Aman","and","chess");
    // Named arguments
    println!("{name} is a {quality}",name="Aman",quality="God");
    // Placeholder traits
    println!("Binary: {:b} Hex:{:x} Octal:{:o}",10,10,10);
    // Placeholder for debug traits
    println!("{:?}",(1,2,7));
    //Basic Math
    println!("10+10 is equal to {} ",10+10);
}