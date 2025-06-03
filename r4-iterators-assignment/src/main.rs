//Write a logic for to filter all the odd numbers and then double every value and return it in a new vector.
fn main() {
    let num=vec![1,2,3,4,5,6,7,8,9];
    let iter=num.iter();
    let iter2=iter.filter(|x| *x%2!=0);
    let iter3=iter2.map(|x| x*2);
    let mut  res=vec![];
    for x in iter3 {
        res.push(x);
    }
    println!("{:?}",res)
}
