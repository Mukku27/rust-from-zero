pub trait Summary {
    fn summarize(&self) -> String;
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    fn summarize(&self) -> String {
        return format!("User {} is {} years old", self.name, self.age);
    }
}

fn notify(item: &impl Summary){
    println!("This is the {}",item.summarize());
}

fn main() {
    let user = User {
        name: String::from("Mukku"),
        age: 20,
    };
    println!("{}", user.summarize());
    notify(&user);
}