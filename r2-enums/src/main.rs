#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    let my_direction = Direction::North;
    let new_direction = my_direction; // This is a copy because simple enums are `Copy`
    println!("{:?}", new_direction); // Use {:?} for Debug formatting
}