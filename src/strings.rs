#![allow(non_snake_case)]
// 1)Primitive str = Immutable fixed-length string somewhere in memory
// 2)String = Growable, heap-allocated data structure - Use when you need to modify or own string data
pub fn run() {
    //1 = Growable like in dsa (arrays)
    let mut hello = String::from("Hello ");
    // Get length
    println!("Length: {}", hello.len());
    // push is used to add only characters
    hello.push('A');
    println!("{}",hello);
    // push_str used to add strings to prexisiting strings
    let mut  _hello = String::from("Hello ");
    _hello.push_str("Aman");
    println!("{}",_hello);
    // Capacity in bytes
    println!("Capacity is {}",_hello.capacity());
    // Check if empty 
    println!("{}",_hello.is_empty());
    //Contains a substring
    println!("Contains Aman: {}",_hello.contains("Aman"));
    //Replace
    println!("Replace : {}",_hello.replace("Aman","World"));
    let new_Hello = _hello.replace("Aman","World");
    println!("{}",new_Hello);
    //Loop through strings by whitespace
    for word in _hello.split_whitespace() {
        println!("{}",word);
    }
    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    // Assertion testing
    assert_eq!(2,s.len());



}