fn main() {

    let x: i32 = 208;
    match x {
        1..=31 => println!("Less than 32"),
        32..=64 => println!("Between 64 and 32"),
        _ => println!("Greater than 64"),
    }

}

