

// This is a simple macro named `greet`.
macro_rules! greet {
    // first arm matches a &str value passed in 
       ($a:expr)=>{
           {
               println!("Hello, {}", $a);
           }
       };
   // Second arm matches no &str argument passed in
       ()=>{
           {
               println!("Hello");
           }
       }
   }
   
fn main() {

    let v: Vec<i32> = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    
    let name: &str = "Dan";
    // This call will output "Hello, Dan!"
    greet!(name);
    // This call will output "Hello!"
    greet!();
}