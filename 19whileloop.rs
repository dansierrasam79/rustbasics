fn main() {

let mut i = 0;
println!("while loop begins...");

while i < 10 {
    i += 1;
    if i == 7 {
        continue;
    }
    println!("{}", i);
}
println!("while loop ends...");

}