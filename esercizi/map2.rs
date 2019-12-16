// Make the test pass by creating a new hashMap where each value added by two

use std::collections::BTreeMap;

fn create_map(map : &BTreeMap<String,i32>) -> BTreeMap<String,i32> {
    
    let mut m  = BTreeMap::new();
    
    for (k, v) in map{
        let s = String::from(k);
        m.insert(s, v+2);
    }
    m
}


#[cfg(test)]
mod tests {

    use super::create_map;
    use std::collections::BTreeMap;
  
    #[test]  
    fn test() {
     
     
     let map = vec![(String::from("1"),1),(String::from("2"),1),(String::from("3"),1)].into_iter().collect();
     
     let expected_map : BTreeMap<String,i32> = vec![(String::from("1"),3),(String::from("2"),3),(String::from("3"),3)].into_iter().collect();
     
    
     assert_eq!(expected_map,create_map(&map));   
     
     
     
     
    }
}
