use std::collections::HashMap;

fn main(){
    
    let mut users:HashMap<String,i32> = HashMap::new();

    users.insert(String::from("Sachin Tendulkar"), 100);
    users.insert(String::from("Virat Kohli"), 83); 
    users.insert(String::from("Ricky Ponting"), 70); 

    let fisrt_cricketer_centuries=users.get("Sachin Tendulkar");
    println!(" sachin has {:?} hunderds in his intl career",fisrt_cricketer_centuries.unwrap());
    
}