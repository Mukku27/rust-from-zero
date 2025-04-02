fn main() {
   
   let x:u8=10; // x is a variable of type unsigned 8-bit integer which is immutable by default 
   println!("x = {}", x);

    let mut y:i32=20; // y is a variable of type signed 32-bit integer which is mutable
    println!("y = {}", y);
    y=30;
    println!("y = {}", y);

    let z:u64=100; // z is a variable of type unsigned 64-bit integer which is immutable by default
    println!("z = {}", z);

    let a:i64=200; // a is a variable of type signed 64-bit integer which is immutable by default
    println!("a = {}", a);

    let b:f32=10.5; // b is a variable of type 32-bit floating point number which is immutable by default
    println!("b = {}", b);
    
}
