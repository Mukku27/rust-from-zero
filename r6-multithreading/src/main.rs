use  std::thread;
use std::time::Duration;


fn main(){
    
    let handle=thread::spawn(||{
        for i in 1..10{
            println!( " {i} from the spawned thread");
            thread::sleep(Duration::from_millis(2));
        }
    });

   handle.join().unwrap();

    for i in 1..5{
        println!("{i} from the main theard");
        thread::sleep(Duration::from_millis(2));

    }

    

}
