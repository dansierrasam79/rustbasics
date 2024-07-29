
fn product_two_numbers(numa: i32, numb: i32)-> i32 {
    numa * numb
}

fn mult_return(nume: i32, numf: i32) -> [i32;2] {
    let a: [i32; 2]= [nume, numf];
     return a
}

fn no_return(numc: i32) {
    println!("Result 3 {}", numc);
}

fn main() {
    // first time use
    
    let a: i32 = 10;
    let b: i32 = 3;
    let result: i32 = product_two_numbers(a, b);
    
    println!("Result {}", result);

    // second time use
    let c: i32 = 12;
    let d: i32 = 34;
    let result2: i32 = product_two_numbers(c,d);
    println!("Result 2 {}", result2);

    // Multiple return type
    let e: i32 = 16;
    let f: i32 = 29;
    let num_array: [i32;2] = mult_return(e,f);
    println!("Array {:?}", num_array);

    // No return type
    let g: i32 = 23;
    no_return(g);
}

