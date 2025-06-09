// The 'a is a lifetime parameter. It means the returned reference
// will live as long as the SHORTER of the lifetimes of str1 and str2.
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        return str1;
    } else {
        return str2;
    }
}

fn main() {
    let ans: &str; // `ans` will hold a reference to a string slice.

    let str1 = String::from("Virat Kohli"); // `str1` OWNS its data ("Virat Kohli").
                                           // This data lives as long as `str1` (until end of `main`).
    {
        // Inner scope starts
        let str2 = String::from("Sachin Tendulkar"); // `str2` OWNS its data ("Sachin Tendulkar").
                                                     // This data lives only as long as `str2` (until end of this inner scope).

        // `longest` is called with references:
        // `&str1` borrows from `str1`.
        // `&str2` borrows from `str2`.
        // The lifetime 'a for `ans` becomes the lifetime of `&str2` (the shorter one).
        ans = longest(&str1, &str2);

        // If `str2` is longer, `ans` now refers to data owned by `str2`.
        // The lifetime of `ans` is tied to this inner scope.

    } // Inner scope ends.
      // `str2` goes out of scope. Its OWED data ("Sachin Tendulkar") is DROPPED (deallocated).

    // ERROR: `str2` does not live long enough.
    // `ans` might be referring to the data previously owned by `str2`, which no longer exists.
    // `ans` has a lifetime tied to the inner scope, but it's being used here, outside that scope.
    // This is a DANGLING REFERENCE, which Rust's ownership and lifetime rules prevent at compile time.
    println!(" longest :{}", ans);
}