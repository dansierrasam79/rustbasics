fn main() {
    
    let mut count: i32 = 0;

    println!("Infinite loop begins...");

    loop { 
        count += 1;
        println!("{}", count);
    
        if count == 10 {
            break;
        }
    }

    println!("Infinite loop ends...");

}