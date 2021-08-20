#![allow(non_snake_case)]
// Structs - Used to create custom data types

// Traditional Struct
// struct Color {
//   red: u8,
//   green: u8,
//   blue: u8,
// }


// struct Color(u8, u8, u8);
struct Color {
        red:u8,
        green:u8,
        blue:u8
    }
// Tuple Struct
struct Boxer(u8,u8,u8);
struct Person {
    first_name :String,
    last_name : String
}
impl Person {
    // Construct Person
    fn new(first:&str,last:&str) -> Person {
        Person {
            first_name : first.to_string(),
            last_name : last.to_string(),

        }
    }
    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}",self.first_name,self.last_name)
    }
    // Set first name
    fn set_first_name(&mut self,last:&str){
        self.first_name = last.to_string();
    }
    fn name_to_tuple(self) -> (String,String) {
        (self.first_name,self.last_name)
    }
}
pub fn run() {
    let mut c = Color {
        red:255,
        green:0,
        blue:0

    };
    c.red = 200;
    println!("Color : {} {} {}",c.red,c.green,c.blue);
// To access the tuple struct
let mut _c = Boxer(0,255,255);
_c.0 = 255;
println!("boxer : {} {} {}",_c.0,_c.1,_c.2);

// Implementing Person struct 
let mut p = Person::new("Deepak","Sharma");
p.set_first_name("Aman");
println!("The full name is {}",p.full_name());
println!("Person {} {}",p.first_name,p.last_name);
println!("{:?}",p.name_to_tuple());

}