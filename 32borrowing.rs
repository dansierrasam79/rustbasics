
fn main() {
    let mut new_string = String::from("Lumos Labs");
    let first_ref = &mut new_string;
    let second_ref = &mut new_string;

    println!("{},{}", first_ref, second_ref);
}