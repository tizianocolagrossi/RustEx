// Make the test pass by calculating the sum of the elements in the values of the map

use std::collections::HashMap;

fn sum(vec : &HashMap<String,i32>) -> i32{
    let mut v = 0;
    for (k, val) in vec{
        v += val;
    }
    v
}


#[cfg(test)]
mod tests {

    use super::sum;
  
    #[test]  
    fn test() {
     
     
     let map = vec![(String::from("1"),1),(String::from("2"),1),(String::from("3"),1)].into_iter().collect();
     
    
     assert_eq!(3,sum(&map));   
     
     
     
     
    }
}
