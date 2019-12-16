// Fix the compilation error by changing the create_user function input to a slice string

fn create_user<'a>(name: &'a str) -> User {
    User { name: name }
}
#[derive(Debug)]
struct User<'b> {
    name: &'b str,
}



fn main (){
    
    let user = create_user(&"test");
    
    
    
   
    println!("{:?}",user);
    
}
