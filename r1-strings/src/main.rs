fn main() {
let my_string=String::from("I am Iron Man");    
println!("The number of characters in the string are {}",string_length(&my_string));
}
fn string_length(s: &str)-> usize{
  return s.chars().count();
}
