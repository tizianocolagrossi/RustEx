#[derive(Debug)]
struct Pair<A,B> {
    first : A,
    second : B
}


impl<A,B> Pair<A,B> {
 
    // create a method with_first that allocate a new struct Pair 
    // where the first field is taken in input, and the second is taken from self
    fn with_first<C>(self, first: C) -> Pair<C,B>{
        Pair{first, second: self.second}
    }
    
}

fn main() {
    
    let p = Pair { first: 5, second: 10 };

    println!("bigger = {:?}", p.with_first(String::from("Test")));				
    
    
    
}

