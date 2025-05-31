use chrono::Local;
use rand::Rng;


fn main() {
  let now = Local::now();
  println!("{}",now);



  let mut rng = rand::rng();
  let n1: u32 = rng.random_range(1..=100); // The ..= creates an inclusive range
  println!("Random u32 between 1 and 100: {}", n1);

  let n2: f64 = rng.random(); // gen() can infer the type for many standard types
  println!("Random f64 between 0.0 and 1.0: {}", n2);

     // Generate a random boolean
     let random_bool: bool = rng.random();
     println!("Random boolean: {}", random_bool);
 
     if random_bool {
         println!("It was true!");
     } else {
         println!("It was false!");
     }

}



