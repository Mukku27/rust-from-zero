fn main(){

    let mut vec=Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    vec.push(6);
    println!("{:?}",vec);
    println!("The even nuumbers in vec :{:?}",is_even(vec.clone()));
    println!("The odd nuumbers in vec :{:?}",is_odd(vec.clone()));
    



    //method-1 for string vectors 
    let mut vec_str: Vec<&str> = Vec::new(); // Explicitly a Vec of string slices
    vec_str.push("aa");                     // Push into vec_str, use string literal
    vec_str.push("abd");                    // Push into vec_str, use string literal
    vec_str.push("hello");
    println!("String slice vector: {:?}", vec_str);

    //method-2 for string vectors
    let mut vec_str: Vec<String> = Vec::new(); // Explicitly a Vec of Strings
    vec_str.push(String::from("aa"));        // Create a String and push
    vec_str.push("abd".to_string());         // Another way to create a String
    vec_str.push(String::from("hello rust"));
    println!("Owned String vector: {:?}", vec_str);

}

fn is_even(vec: Vec<i32>)->Vec<i32> {
    let mut new_vec=Vec::new();
    for val in vec {
        if val%2==0{
            new_vec.push(val);
        }
    }

    return new_vec;
}

fn is_odd(vec: Vec<i32>)->Vec<i32> {
    let mut new_vec=Vec::new();
    for val in vec {
        if val%2!=0{
            new_vec.push(val);
        }
    }

    return new_vec;
}

