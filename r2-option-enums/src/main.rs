/*Basic Structure  of the option enum
 pub enum Option<T>{
        None,
        Some(T),

     }
*/

enum CustomOption {
    Some(i32),
    None,
}

fn main() {
    let index = find_first_a(String::from("preeta"));

    match index {
        CustomOption::Some(value) => println!("index is {}", value),
        CustomOption::None => println!("a not found"),
    }
}

fn find_first_a(s: String) -> CustomOption {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return CustomOption::Some(index as i32);
        }
    }
    return CustomOption::None;
}
