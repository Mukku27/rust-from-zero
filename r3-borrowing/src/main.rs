fn main() {
    let s1 = String::from("hello");

    // Immutable borrowing
    let len = calculate_length(&s1); // Pass a reference to s1
    println!("The length of '{}' is {}.", s1, len); // s1 is still valid and usable

    // Mutable borrowing
    let mut s2 = String::from("world");
    change_string(&mut s2); // Pass a mutable reference to s2
    println!("Changed string: {}", s2); // s2 has been modified

    // Multiple immutable borrows are fine
    let r1 = &s2;
    let r2 = &s2;
    println!("r1: {}, r2: {}", r1, r2);
    // At this point, r1 and r2 go out of scope.

    // Cannot have a mutable borrow while immutable borrows exist (or vice-versa)
    // let r3 = &s2;
    // let r4_mut = &mut s2; // COMPILE ERROR: cannot borrow `s2` as mutable because it is also borrowed as immutable
    // println!("r3: {}", r3);

    // Only one mutable borrow at a time
    let r_mut1 = &mut s2;
    // let r_mut2 = &mut s2; // COMPILE ERROR: cannot borrow `s2` as mutable more than once at a time
    r_mut1.push_str(" again!");
    println!("s2 after another mutable borrow: {}", r_mut1); // Or println!("{}", s2);
    // r_mut1 goes out of scope here, so s2 can be borrowed again.

    let r_mut3 = &mut s2;
    r_mut3.push_str(" and again!");
    println!("s2 finally: {}", s2);

    // Example of a dangling reference (prevented by Rust)
    // let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize { // s is an immutable reference
    // s.push_str(" world"); // COMPILE ERROR: cannot borrow `*s` as mutable
    s.len()
} // s goes out of scope, but the data it refers to is NOT dropped.

fn change_string(some_string: &mut String) { // some_string is a mutable reference
    some_string.push_str(", Rust!");
} // some_string goes out of scope.

// fn dangle() -> &String { // This function's return type contains a borrowed value,
//     let s = String::from("dangle");
//     &s // but `s` is owned by `dangle`
// } // `s` goes out of scope and is dropped. Its memory is gone.
  // Returning a reference to it would be a dangling reference.
  // Rust prevents this with a compile-time error.