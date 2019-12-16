// Refactor this code in order to remove the code duplication
use std::ops::Add;


struct Pair<T> {
    x: T,
    y: T
}

impl<T: Add> Pair<T> {
    fn sum(self)->T::Output{
        self.x + self.y
    }
}



fn main (){
    
    let p = Pair { x : 1, y : 1};
    
    println!("Sum : {}" , p.sum());
    
    let p = Pair { x : 1.0, y : 1.1};
    
    println!("Sum : {}" , p.sum());
}

