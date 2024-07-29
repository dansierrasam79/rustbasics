#[derive(Debug)]
enum CourseName {
    Rust,
    Math,
    English,
    Sanskrit
}
#[derive(Debug)]
enum CourseStatus {
Distinction,
    Pass,
    Fail,
    Suspended,
}
#[derive(Debug)]
enum Gender {
    Male,
    Female,
}
#[derive(Debug)]
struct Student {
    name:String,
    gender:Gender,
    course: CourseName,
    status: CourseStatus,
    marks: i32
} 

fn main() {

    let student = Student {
        name:String::from("Maurice"),
        gender:Gender::Male,
        course:CourseName::Rust,
        status:CourseStatus::Pass,
        marks:36,
    };

    println!("Student Name: {}", student.name);
    println!("Student Gender: {:?}", student.gender);
    println!("Course Taken: {:?}", student.course);
    println!("Course Status: {:?}", student.status);
    println!("Marks: {:?}", student.marks);


}