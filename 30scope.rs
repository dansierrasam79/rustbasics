fn main() {
    let x: i32 = 12;
    {
        let y: i32 = 23;
    }
    println!("X: {}", x);
    println!("Y: {}", y);
}