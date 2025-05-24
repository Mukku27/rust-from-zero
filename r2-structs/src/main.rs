struct User{
    frist_name:String,
    last_name: String,
    age:i32,
    email:String,
}
fn main() {
    let user=User{
        frist_name : String::from("Tony"),
        last_name : String::from("Stark"),
        age:45,
        email:String::from("jarivs@friday.com"),

    };
    println!("{}",user.frist_name);
    println!("{}",user.last_name);
    println!("{}",user.age) ;
    println!("{}",user.email);

}
