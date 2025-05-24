fn main() {
    let x:u8=12;
    println!("{}",is_even(x.into()));
    if x%2==0 
     { println!("{} is  even number",x); }
}
fn is_even(num:i32)->bool {
    if num%2==0{ 
     return true ;}
    return false; 
    
}