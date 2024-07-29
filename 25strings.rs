
// Read only string that remains the same throughout the program
const WHO_WE_ARE: &'static str = "Lumos Labs";

fn main(){
    println!("Constant string: {}", WHO_WE_ARE);

    // Borrowable string that is of fixed length
    let who_we_are_literal: &str = "Lumos Labs";
    println!("Owned string: {}", who_we_are_literal);
    
    //Mutable and growable whose size is unknown at compile size
    let mut who_we_are_growable = String::from("Lumos");
    println!("Growable string: {}",who_we_are_growable);
    
    // Example of creating a string slice
    let who_we_are_slice: &str = &who_we_are_growable[0..3];   
    println!("Slice of growable string: {}", who_we_are_slice);

    // Example of using the .push_str() on a String object 
    who_we_are_growable.push_str(" Labs");
    println!("Changed growable string: {}",who_we_are_growable);

    // Iterating over a growable string
    
    for char in who_we_are_growable.chars() {
        println!("{}", char);
    }

}