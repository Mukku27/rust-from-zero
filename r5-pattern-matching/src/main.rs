fn main() {
    // Pattern matching with numbers
    let number = 3;
    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 => println!("This is a prime number!"),
        4..=10 => println!("Between 4 and 10"),
        _ => println!("Some other number"),
    }

    // Matching with enums
    #[derive(Debug)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let dir = Direction::East;

    match dir {
        Direction::North => println!("Heading North"),
        Direction::South => println!("Heading South"),
        Direction::East => println!("Heading East"),
        Direction::West => println!("Heading West"),
    }

    // Destructuring tuples
    let point = (0, 7);
    match point {
        (0, y) => println!("Point is on the y-axis at y = {}", y),
        (x, 0) => println!("Point is on the x-axis at x = {}", x),
        (x, y) => println!("Point is at ({}, {})", x, y),
    }

    // Matching Option<T> with if let
    let some_value = Some(42);

    if let Some(x) = some_value {
        println!("We got a value: {}", x);
    } else {
        println!("Nothing here");
    }

    // Matching Result<T, E> with match
    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err(String::from("Division by zero"))
        } else {
            Ok(a / b)
        }
    }

    match divide(10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(5, 0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
