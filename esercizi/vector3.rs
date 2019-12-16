// Make the test pass by calculating the sum of the elements in the vector


fn sum(vec : &Vec<i32>) -> i32{
    let mut v = 0;
    for i in vec{
        v+=i;
    }
    v
}


#[cfg(test)]
mod tests {

    use super::sum;
  
    #[test]  
    fn test() {
     
     let vec = vec![1,2,3];
     assert_eq!(6,sum(&vec));   
     
    }
}
