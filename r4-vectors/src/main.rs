fn main(){

    let mut vec=Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("{:?}",vec);


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


