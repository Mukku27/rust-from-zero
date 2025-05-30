use std::f64::consts::PI; // Import the PI constant for better precision

enum Shape {
    Rectangle(f64, f64), // Represents (length, width)
    Circle(f64),         // Represents (radius)
}

// Function to calculate area
// It takes ownership of the Shape (or a copy, since f64 is Copy)
fn calculate_area(shape: Shape) -> f64 {
    match shape {
        // When matching tuple variants, you bind the inner values to new names
        Shape::Rectangle(length, width) => length * width, 
        Shape::Circle(radius) => PI * radius * radius,   
}
}

fn main() {
    let rect = Shape::Rectangle(1.0, 2.0);
    // Pass the variable `rect` directly
    
    println!("Area of rectangle: {}", calculate_area(rect));
    // Note: `rect` is moved here because Shape (containing f64s) is Copy by default,
  

    let circle = Shape::Circle(2.0);
    // Pass the variable `circle` directly
    println!("Area of circle: {}", calculate_area(circle));
}