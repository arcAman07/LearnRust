#![allow(non_snake_case)]
pub fn run() {
    // Integer Data type default is i32
    let x = 272;
    // Float Data type default is f64
    let y = 7.2;
    //Add explicit data type
    let z: i64 = 982739208372;
    //Find max size
    println!("Max i32 is {}",std::i32::MAX);
    println!("Max i64 is {}",std::i64::MAX);
    // Boolean
    let is_active = true;
    let face = '\u{1F600}';
    println!("{:?}",(x,y,z,is_active,face));



}