fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // arr is a variable of type array of signed 32-bit integers which is immutable by default
    println!("The value of arr is: {:?}", arr);
    println!("The first value of arr is: {}", arr[0]); // accessing the first element of the array
    println!("The second value of arr is: {}", arr[1]); // accessing the second element of the array
    println!("The third value of arr is: {}", arr[2]); // accessing the third element of the array
    println!("The fourth value of arr is: {}", arr[3]); // accessing the fourth element of the array
    println!("The fifth value of arr is: {}", arr[4]); // accessing the fifth element of the array


    let mut arr2: [i32; 5] = [1, 2, 3, 4, 5]; // arr2 is a variable of type array of signed 32-bit integers which is mutable
    println!("The value of arr2 is: {:?}", arr2);
    arr2[0] = 10; // changing the first element of the array
    arr2[1] = 20; // changing the second element of the array
    arr2[2] = 30; // changing the third element of the array
    println!("The value of arr2 is: {:?}", arr2);

}
