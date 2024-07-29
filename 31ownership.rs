#[warn(unused_variables)]
fn rules() {
    let value: i32 = 5;
    //Ownership rule does not apply to primitive values because value = 5 gets copied to value2 as well 
    let value2 = value;
    //No ownership error
    println!("Values: {} {}", value, value2);
    
    
    //Only one owner permitted, so str_value is dropped

    let str_value = String::from("Dan");
    // Borrow the value stored in str_value using the & operator
    let str_value2 = &str_value;
    println!();
    // Will not generate an error
    println!("String value: {}", str_value);
    // Value has moved to str_value2
    println!("String value 2: {}", str_value2);
    println!();
    


}

fn main() {
    let float_value: f64 = 3.22;
    let str_value = String::from("Variable In Scope");
    println!("Float: {}", float_value);
    println!("String literal: {}", str_value);
    rules();
}
// after the closing curly brace, the scope of main is finished... so the values of both float_value and str_value is dropped  