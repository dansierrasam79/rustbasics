
fn get_int_ref<'a>(int1: &'a i32, int2: i32) -> &'a i32 {
    println!("{}", int2);
    int1
}

fn main() {
    let first_int: i32 = 15;
    let sec_int: i32 = 25;
    let result_ref = get_int_ref(&first_int, sec_int);
    println!("{}", result_ref);
}


