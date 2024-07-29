

fn diverging()-> ! {
    panic!("This is what a diverging function does!!!")
}

fn main() {
    diverging()
}