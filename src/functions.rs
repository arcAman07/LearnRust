#![allow(non_snake_case)]
pub fn run() {
    greeting("Hello","Aman");
    // Bind function values to variables
    let get_sum = _add(2,7);
    println!("Sum : {}",get_sum);
    // Closure- You cab use outside variables with it which is not possible in the case of functions
    let n3:i32 = 12;
    // As you can see we are adding n3 here to the function as an external variable
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3  ;
    println!("C Sum is {}",add_nums(4,11));

}
fn greeting(greet: &str, name: &str){
    println!("{} {}, nice to meet you",greet,name);
}
// -> Here is used to return the value n1+n2 and i32 tells the return datatype
fn _add(n1: i32, n2:i32) -> i32 {
    // values to be returned are written without semi colon(;)
    n1+n2
}