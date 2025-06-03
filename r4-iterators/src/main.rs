fn main() {
 //ITERATORS
    
   //1.iterating  using for loop
   let vec_num=vec![1,2,4,5,3,5,6,7];
   for num in vec_num{
    println!("{}",num);
   }

   //2.iterating after creating an Iterator
   println!("iterating after creating an Iterator");
   let num=vec![1,2,3,4,5,6,7];
   let iter=num.iter();

   for value in iter {
    println!("{}",value);
   }
}
