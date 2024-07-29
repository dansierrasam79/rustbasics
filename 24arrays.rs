fn main() {

let liverpool_2022_stats: [i32;5] = [38, 19, 75, 47, 67];

//Display individual values in tuple using index values
println!("Games played: {}", liverpool_2022_stats[0]);
println!("Games won: {}", liverpool_2022_stats[1]);
println!("Goals for: {}", liverpool_2022_stats[2]);
println!("Goals against: {}", liverpool_2022_stats[3]);
println!("Total points: {}", liverpool_2022_stats[4]);

let mut liverpool_2022_stats_2: [i32;5] = [38, 19, 75, 47, 67];
println!("Liverpool FC Stats: {:?}", liverpool_2022_stats_2);
liverpool_2022_stats_2[1] = 20;
println!("Changed Liverpool FC Stats: {:?}", liverpool_2022_stats_2);
println!();
// Iterate through the liverpool_2022_stats using another variants of the for loop
for element in liverpool_2022_stats {
    println!("Element in array: {}", element);
}
println!();

// Extract a slice from the array using the []
println!("{:?}", &liverpool_2022_stats[0..3]);


println!();
}