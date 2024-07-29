

fn product_two_numbers(numa: i32, numb: i32)-> i32 {
    numa * numb
}
// fn syntax for own
fn display_name(name: String){
    println!("In function display_name: {}", name);
    name
}

//fn syntax for borrow
fn display_name(name: &String){
    println!("In function display_name: {}", name);
    name
}

fn main() {

    //Compound values ownership
    let name: String = String::from("Dan");
    let result2: String = display_name(name);
    // name cannot be displayed below as display_name owns the value
    //println!("Name: {}", name);
    println!("In main function: {}", result2)

    //Compound values borrowing
    let name: String = String::from("Dan");
    let result2: String = display_name(&name);
    // name can be displayed below as display_name has borrowed the value
    println!("Name: {}", name);
    //println!("In main function: {}", result2)

    // Scalar values own and borrow
    let a: i32 = 10;
    let b: i32 = 3;
    let result: i32 = product_two_numbers(a, b);
    println!("{}", a);
    println!("{}",b);
    println!("Result {}", result);
}