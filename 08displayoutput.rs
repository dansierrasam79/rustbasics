

fn main() {
    let a_array: [i32; 5] = [1,2,3,4,5];
    let mut total: i32 = 0;
    let meanie: i32;
    for el in a_array{
        print!("Element: {} ", el);
        total += el;
    }
    println!("");
    println!("Sum: {}", total);
    meanie = total/5;
    println!("Average: {}", meanie);
}

