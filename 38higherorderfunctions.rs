

fn product(a: i32, b: i32) -> i32 {
    a * b
}

fn compute_prod<F>(x: i32, y: i32, prod: F) -> i32 
                     where F: Fn(i32, i32) -> i32 {
    prod(x, y)
}

fn main() {
    println!("{}", compute_prod(10, 20, product));
}