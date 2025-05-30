struct Rect{
    length:u32,
    width:u32,
}
impl Rect{
    fn area(&self)->u32
    {
        return self.width*self.length;
    }
    fn perimeter(&self)->u32
    {
        return 2*(self.width+self.length);
    }
    fn debug()->u32  
    {
        return 1 ;
    }
}
fn main() {
   let rec=Rect{
    length:30,
    width:40,
   };
   println!("The total area of the Rectangle = {}",rec.area());
   println!("The total perimeter of the Rectangle = {}",rec.perimeter());
   println!("{}",Rect::debug());
}
