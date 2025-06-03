fn main() {
  let name=String::from("Virat Kohli");
  let first_name=&name[0..5];//string slice &str
  let second_name=&name[6..11];//string slice &str
  println!("{}",first_name);
  println!("{}",second_name);
}
