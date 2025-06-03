fn main() {
    //Map
    
    let num=vec![1,2,3,4,5,6,7];
    let iter=num.iter();
    println!("map function for x+1");
    let iter2=iter.map(|x| x+1);
    for x in iter2 {
        println!("{}",x);
    }

    //Filter

    let nums=vec![1,2,3,4,5,6];
    let iters=nums.iter();
    println!("Filter function for even");
    let iters2=iters.filter(|x| *x%2==0);
    for x in iters2{
        println!("{}",x);
    }

}

