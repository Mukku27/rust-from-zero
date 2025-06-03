fn main() {
   let arr=[1,2,3,4,5,6,7];
   let arr_slice=&arr[0..3];
   println!("array={:?}",arr);
   println!("slice of the array={:?}",arr_slice);
}
