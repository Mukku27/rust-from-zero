/*
Result enum is either returns result(res) or error(err) 
 retuns OK value or ERR value
Result enum is way to error handling in Rust

*/


use std::fs::read_to_string;

fn main() {
    // Corrected way to call read_to_string
    let result = read_to_string("r2-result-enums/src/a.txt");

    match result {
        Ok(data) => println!("{}", data),
        Err(err) => println!("Error while reading the file: {}", err),
    }
}
