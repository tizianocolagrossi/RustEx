// Make the test pass by changing the element at position `pos` with the value provided in the function



fn change_single_item(pos : usize, value : i32, vec : &mut Vec<i32>) {
    vec[pos]=value;
}


#[cfg(test)]
mod tests {

    use super::change_single_item;
  
    #[test]  
    fn test() {
     
     
     let mut vec = vec![1,2,3];
     
     change_single_item(0,0,&mut vec);
     
     assert_eq!(vec![0,2,3],vec);   
     
     change_single_item(1,0,&mut vec);
     
     assert_eq!(vec![0,0,3],vec);   
     
     change_single_item(2,0,&mut vec);
     
     assert_eq!(vec![0,0,0],vec);   
     
     
    }
}
