use std::{error, string::String};

fn divide_int_or_error(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        return Err(String::from("{} : Can't Divide by zero."));
    }
    return Ok(x / y);
}

fn main() {
    println!("Hello, world!");
    match divide_int_or_error(5, 0) {
        Ok(result) => println!("{}", result),
        Err(e) => println!("{}", e),
    }
    let mut x: i32 = 5;

    unsafe {
        let y: *mut i32 = &mut x as *mut i32;
        *y = 10;
    }

    println!("x: {}", x);
}
