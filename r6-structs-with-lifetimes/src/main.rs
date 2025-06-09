struct User<'a>{
    name :&'a str,
}

fn main(){

    let user;
    {
        let name=String::from("Mukesh");
        user=User{name:&name};
        println!("{}",user.name);
    }
    //println!("{}",user.name);
   // ❌ Dangling reference error at compile time 
   
}