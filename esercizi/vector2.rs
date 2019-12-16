// Make the test pass by adding 2 to each element in the array



fn add_two(vec : &mut Vec<i32>){
    for i in vec{
        *i += 2;
    }

}


#[cfg(test)]
mod tests {

    use super::add_two;
  
    #[test]  
    fn test() {
     
     let mut vec = vec![0,2,4];
     
     add_two(&mut vec);
     
     assert_eq!(vec![2,4,6],vec);   
     
     add_two(&mut vec);
     
     assert_eq!(vec![4,6,8],vec);   
    }
}
