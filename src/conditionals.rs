#![allow(non_snake_case)]
pub fn run() {
    let age: i32=27;
    let check_id: bool = true;


    // If/Else
    // && is the 'and' operator
    //  || is the 'or' operator
    if age >= 21 && check_id{
        println!("You are an adult");
    }
    else if age<21 && check_id{
        println!("You are a kid");
    }
    else {
        println!("I need to see your id");
    }

    //shorthand if statements
    let is_of_age = if age>=2 { true} else { false };
    println!("{}",is_of_age);
}
