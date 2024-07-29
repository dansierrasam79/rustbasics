fn add_two_numbers(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let add_value: i32 = add_two_numbers(6,2);
    println!("{}", add_value);
}