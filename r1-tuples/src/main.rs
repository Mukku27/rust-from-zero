fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is: {:?}", tup);
    println!("The first value of tup is: {}", tup.0);   
    println!("The second value of tup is :{}",tup.1);
    println!("The third value of tup is :{}",tup.2);
    let (x, y, z) = tup;
    // destructuring the tuple
    // x is a variable of type i32 which is immutable by default
    // y is a variable of type f64 which is immutable by default
    // z is a variable of type u8 which is immutable by default
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

}
