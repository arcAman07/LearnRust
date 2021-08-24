#![allow(non_snake_case)] 
struct _Name {
    first_name : String,
    last_name : String
}
impl _Name {
    pub fn join(first:&str,last:&str) -> _Name {
        _Name {
            first_name : first.to_string(),
            last_name : last.to_string(),

        }
        
    }
    fn fullName(&self) -> String {
        println!("The full name is {} {}",self.first_name,self.last_name)

    }


}
pub fn run() {
    let mut a1 = _Name::join("Aman","Sharma");
    a1.fullName();

}


