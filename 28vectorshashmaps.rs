
fn main() {
    use std::collections::HashMap;
    // Declare a Vector in Rust
    let numbers_growable: Vec<i32> = vec![1,2,3];
    // Access vector items by index position 
    println!("First item: {:?}", numbers_growable[0]);
    println!("Second item: {:?}", numbers_growable[1]);
    println!("Third item: {:?}", numbers_growable[2]);

    // Iterate over the values of a Vector
    for element in &numbers_growable {
        println!("{}", element);
    }

    // Example of using a Vector method
    println!("Length: {}", numbers_growable.len());

    // Declare a Hash Map in Rust
    let mut football_player_stats: HashMap<String, i32> = HashMap::new();

    //Add key-value pairs to the HashMap

    football_player_stats.insert(String::from("Messi"), 850);
    football_player_stats.insert(String::from("Ronaldo"), 819);
    football_player_stats.insert(String::from("Lewandowski"), 604);
    football_player_stats.insert(String::from("Suarez"), 536);
    println!("{:?}", football_player_stats);

    //Read key-value pairs from a HashMap
    let player_name = String::from("Suarez");
    println!("{}", football_player_stats.get(&player_name).copied().unwrap_or(0));

    //Delete a key-value pair from a Hashmap
    let player_name_2 = String::from("Suarez");
    football_player_stats.remove(&player_name_2);
    println!("{:?}", football_player_stats);

    //Loops throughout the HashMap to print all key-value pairs
    println!();
    for (player, goals) in &football_player_stats{
        println!("{}:{}", player, goals);
    }
    println!();



}