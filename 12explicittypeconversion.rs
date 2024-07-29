
fn main() {

    // Convert between primitive data types, float to integer
    let num: f64 = 5.82;
    let cast_to_integer_type = num as i32;
    println!("Float value: {}", num);
    println!("Integer value: {}", cast_to_integer_type);

    // Convert from non-primitive to primitive data type, String to integer
    let new_string = String::from("2.71828");
    let euler_const: f64 = new_string.parse().unwrap();
    println!("Euler's Constant {}", euler_const); 

}
