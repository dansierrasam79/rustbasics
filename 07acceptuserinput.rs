use std::io;

fn use_stdin() {
    println!("Enter your name: ");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    println!("Hey, {}", user_input);
}

fn use_args() {
    let mut arguments_vec = Vec::new();
    let cmd_line = std::env::args();
    println!("No of arguments: {}", cmd_line.len()-1);
    
    for arg in cmd_line {
        arguments_vec.push(arg);
    }

    println!("Hey, {}", arguments_vec[1]);
}

fn use_stdin2() -> io::Result<String>{
    println!("Enter your name: ");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;
    println!("Hey, {}", user_input);
    Ok(user_input)
}

fn main() {
    let _ = use_stdin();
    let _ = use_args();
    let _ = use_stdin2();
}