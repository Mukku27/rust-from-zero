fn main() {

let num:i32=10;
println!("{}",fibo(num));

}
fn fibo( num:i32)->i32
{  let mut first:i32=0;
   let mut second:i32=1;
   if num==0{
    return first;
   }
   if num==1{
    return second;
   }
   for _ in 1..num-2{
    let temp=second ;
    second=first+second;
    first =temp;
   }
   return second;


}