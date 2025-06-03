fn main() {
 //ITERATORS
    
   //1.iterating  using for loop
   let vec_num=vec![1,2,4,5,3,5,6,7];
   for num in vec_num{
    println!("{}",num);
   }

   //2.iterating after creating an Iterator
   //iterators borrow the  particular vector and dont consume it and original vector is still valid
   println!("iterating after creating an Iterator");
   let num=vec![1,2,3,4,5,6,7];
   let iter=num.iter();

   for value in iter {
    println!("{}",value);
   }

   //3.Muttable iterators method  IterMut()

   let mut  num_mut =vec![1,2,3,4,5,6,7];
   println!("before IterMut original: {:?}",num_mut);
   let iter=num_mut.iter_mut();

   for value in iter {
   *value=*value+1;
   }
   println!("After IterMut +1 :{:?}",num_mut);


   //4.iterating using .next
   println!("iterating using .next");
   let nums=vec![1,2,3,4,5,6,7];
   let mut iter =nums.iter();

   while let Some(val)=iter.next(){
    println!("{}",val);
   }
   

   //5.IntoIter : convert the collection into an iterator that takes the ownership of the collection

   let nums_into=vec![1,2,3,4,5,6,7];
   let iter =nums_into.into_iter();

   for value in iter {
    println!("{}",value);
   }

  // println!("{:?}",nums_into);
  // throws error because into_iter iterator that takes the ownership of the collection

}
