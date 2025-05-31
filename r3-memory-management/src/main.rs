// This function demonstrates stack allocation
fn stack_function() {
    println!("\n--- Inside stack_function ---");

    // These variables are allocated on the stack for `stack_function`
    let a: i32 = 10;         // `a` (4 bytes for i32) is on the stack
    let b: bool = true;      // `b` (1 byte for bool) is on the stack
    let c: [u8; 3] = [1, 2, 3]; // `c` (array of 3 u8, 3 bytes) is on the stack

    println!("Stack variable 'a': {} (value)", a);
    println!("Stack variable 'b': {} (value)", b);
    println!("Stack variable 'c': {:?} (value)", c);

    // We can print their memory addresses (for demonstration, not typically needed)
    // The actual addresses will vary each time you run.
    println!("Address of 'a' (on stack): {:p}", &a);
    println!("Address of 'b' (on stack): {:p}", &b);
    println!("Address of 'c' (on stack): {:p}", &c);

    // When `stack_function` ends, 'a', 'b', and 'c' are automatically
    // removed from the stack (deallocated).
    println!("--- Exiting stack_function ---");
}

// This function demonstrates heap allocation using Box
fn heap_function() {
    println!("\n--- Inside heap_function ---");

    // `Box::new(value)` allocates memory on the heap for `value`
    // and returns a "smart pointer" (`boxed_num`) that lives on the stack.
    // This pointer "points to" the data (500) on the heap.
    let boxed_num: Box<i32> = Box::new(500);

    // `my_string` is a `String` type. `String` manages its character data on the heap.
    // The `String` struct itself (pointer, length, capacity) lives on the stack.
    let my_string: String = String::from("Hello from the heap!");

    println!("Value pointed to by `boxed_num` (on heap): {}", *boxed_num); // Use * to dereference
    println!("`my_string` (data on heap): \"{}\"", my_string);

    // Addresses:
    // `&boxed_num` is the address of the Box pointer on the stack.
    // `boxed_num.as_ptr()` (or similar for raw pointer) would give the heap address.
    println!("Address of `boxed_num` pointer (on stack): {:p}", &boxed_num);
    println!("Address of `my_string` struct (on stack): {:p}", &my_string);
    // To see the heap address where String's data is, it's a bit more involved:
    println!("Address of `my_string`'s internal data (on heap): {:p}", my_string.as_ptr());


    // When `heap_function` ends:
    // 1. `my_string` goes out of scope. Its `drop` method is called,
    //    which frees the character data on the heap.
    // 2. `boxed_num` goes out of scope. Its `drop` method is called,
    //    which frees the `i32` (500) on the heap.
    // The stack variables (`boxed_num` pointer, `my_string` struct) are also popped.
    println!("--- Exiting heap_function ---");
}

// A function that takes ownership of heap data and returns it
fn create_data_on_heap() -> Box<i32> {
    println!("\n--- Inside create_data_on_heap ---");
    let data = Box::new(99);
    println!("Created data on heap: {} at address (stack pointer to heap): {:p}", *data, &data);
    println!("--- Exiting create_data_on_heap, returning ownership ---");
    data // Ownership of the Box (and thus the heap data) is returned
}

fn main() {
    println!("--- Starting main ---");

    // Stack variables in main
    let main_stack_var: i64 = 12345;
    println!("main_stack_var: {} (on main's stack) at {:p}", main_stack_var, &main_stack_var);

    stack_function(); // Call function using its own stack frame

    // After stack_function returns, its stack variables are gone.
    // `main_stack_var` is still here.
    println!("\nBack in main, main_stack_var still exists: {}", main_stack_var);

    heap_function();  // Call function that uses the heap

    // After heap_function returns, its heap allocations have been freed by Rust.

    let returned_heap_data: Box<i32> = create_data_on_heap();
    println!("\nIn main, received returned_heap_data: {} (value on heap, pointer on stack {:p})", *returned_heap_data, &returned_heap_data);


    println!("\n--- Exiting main ---");
    // When main ends:
    // - `main_stack_var` is popped from the stack.
    // - `returned_heap_data` (Box) goes out of scope, its `drop` method is called,
    //   freeing the heap memory it points to.
}