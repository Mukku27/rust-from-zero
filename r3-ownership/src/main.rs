fn main() {
    // s1 owns the String "hello"
    let s1 = String::from("hello");
    println!("s1: {}", s1); // s1 is valid here

    // Ownership of the String data is MOVED from s1 to s2
    let s2 = s1;

    // println!("s1 after move: {}", s1); // This would cause a compile-time error: value borrowed here after move
    println!("s2 (owns the data now): {}", s2); // s2 is valid, s1 is not

    takes_ownership(s2); // s2's ownership is MOVED into the function

    // println!("s2 after function call: {}", s2); // Compile-time error: s2 moved

    let x = 5; // i32 is a simple type, it's COPIED (not moved)
    let y = x;
    println!("x: {}, y: {}", x, y); // Both x and y are valid

    makes_copy(x); // x is COPIED into the function
    println!("x after function call (still valid): {}", x); // x is still valid

    let s3 = gives_ownership(); // s3 receives ownership from the function
    println!("s3 (received ownership): {}", s3);

    let s4 = String::from("world");
    let s5 = takes_and_gives_back(s4); // s4's ownership moved in, new ownership returned to s5
    // println!("s4 after takes_and_gives_back: {}", s4); // Compile-time error
    println!("s5 (got ownership back): {}", s5);

    // If you want to keep the original and also pass it, you can clone
    let original = String::from("clone me");
    let cloned_version = original.clone(); // Deep copy of the String data

    takes_ownership(cloned_version); // cloned_version is moved
    println!("original (still valid after clone and move of clone): {}", original); // original is still valid

} // s3, s5, original go out of scope. `drop` is called, memory is freed.

fn takes_ownership(some_string: String) { // some_string takes ownership
    println!("Inside takes_ownership: {}", some_string);
} // some_string goes out of scope, `drop` is called. Memory is freed.

fn makes_copy(some_integer: i32) { // some_integer gets a copy
    println!("Inside makes_copy: {}", some_integer);
} // some_integer (the copy) goes out of scope.

fn gives_ownership() -> String {
    let a_string = String::from("yours");
    a_string // Ownership is returned
}

fn takes_and_gives_back(a_string: String) -> String {
    println!("Inside takes_and_gives_back, received: {}", a_string);
    a_string // Ownership is returned
}