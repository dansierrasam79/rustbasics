
fn main() {

    let x: i32 = 208;
       
    if x < 32 {
        println!("Less than 32");
    }

    else if (x<64) && (x>32) {
        println!("Between 32 and 64");
    }

    else {
        println!("Greater than 64");
    }

}