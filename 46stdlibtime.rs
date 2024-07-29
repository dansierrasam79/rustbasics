
fn prod_two_nums(x: i32, y:i32)->i32{
    x+y
}

fn main(){
    use std::time::Instant;
    let now = Instant::now();
    let a:i32 = 5;
    let b:i32 = 10;
    // Calling the function as above
    prod_two_nums(a,b);
    
    let elapsed_time = now.elapsed();
    println!("Running prod_two_nums() took {} seconds.", 
                                elapsed_time.as_secs());
}


