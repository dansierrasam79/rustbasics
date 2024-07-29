
fn main() {

    let student_record: (&str, f64, u32) = ("Programming in Rust", 3.2, 25);
    println!("Student record: {:?}", student_record);

    //Display individual values in tuple using index values
    println!("Course name: {}", student_record.0);
    println!("CGPA: {}", student_record.1);
    println!("Age: {}", student_record.2);

    // Working with a mutable tuple
    let mut student_record2: (&str, f64, u32) = ("Programming in Rust", 2.2, 32);
    println!("Student record: {:?}", student_record2);
    student_record2.1 = 3.0;
    println!("Changed student record: {:?}", student_record2);

}