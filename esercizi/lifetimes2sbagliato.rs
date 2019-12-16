
fn create_user<'a>(name: &'a str) -> User {
    User { name: name }
}
#[derive(Debug)]
struct User<'b> {
    name: &'b str,
}



fn main (){
    
   let user =  {
       let name = String::from("name"); 
        let user = create_user(&name);
        user
    };
    
    
    
   
    println!("{:?}",user);
    
}
