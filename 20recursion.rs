fn fact(n: i32) -> i32 {
    if n == 1 {
        n
    }

    else {
        n*fact(n-1)
    }

}

fn main() {
    let n_val: i32 = 5;
    println!("Factorial: {}", fact(n_val));
}