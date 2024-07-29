
// the data structure Human
struct Human
 {
  name:String,
  email:String,
}

// Implementing functions using the impl keyword for the Human struct  
impl Human {
    // Method introduces the human
     fn say_hello(&self) -> String{
        let mut hello: String = String::from("Hello, my name is ");
        hello.push_str(&self.name);
        hello
     }
      
     // Method involves the human giving his or her email
     fn give_email(&self){
        println!("Here is my email address: {}", self.email)
     }
      
   }

   fn main() {
    let human = Human {
        name: "Dan".to_string(),
        email: "dan@abc.com".to_string(),
    };
    println!("{}", human.say_hello());
    human.give_email();
   
}