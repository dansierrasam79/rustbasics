pub mod compute_nums {

    pub fn prod_nums(x: i32, y:i32) {
        println!("Product: {}", x+y);
    }
}

use compute_nums::prod_nums;

fn main(){
    prod_nums(5,6);
}


