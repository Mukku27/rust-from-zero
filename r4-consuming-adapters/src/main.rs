fn main() {
    let num=vec![1,2,3,4,5,6,7];
    let iter=num.iter();

   // let sum:i32=iter.sum();
   
    println!("{}",iter.sum::<i32>())
    //the iterator is consumed here the sum function which is called the consuming adapter

    
}
