fn main() {
  let name=String::from("Virat Kohli");
  let first_name=&name[0..5];//string slice &str
  let second_name=&name[6..11];//string slice &str
  println!("{}",first_name);
  println!("{}",second_name);


  //most common  types of string

  let name = String::from("hello world"); // String type
  let string_slice = &name; // Has a `view` into the original string/is a reference
  let string_literal = "hello"; // literal is also an &str but it points directly to an address in the binary

  println!("{}",string_slice);
  println!("{}",string_literal);
}
