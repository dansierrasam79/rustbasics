#[derive(Debug)]
enum Sports {
    Football,
    Rugby,
    Hockey,
    Basketball,
    Cricket
}

fn main() {

    let dan_fav:Sports = Sports::Rugby;
    let suzy_fav: Sports = Sports::Cricket;
    
    match dan_fav {
        Sports::Football => println!("Dan likes {:?}", Sports::Football),
        Sports::Rugby => println!("Dan likes {:?}", Sports::Rugby),
        Sports::Hockey => println!("Dan likes {:?}", Sports::Hockey),
        Sports::Basketball => println!("Dan likes {:?}", Sports::Basketball),
        Sports::Cricket => println!("Dan likes {:?}", Sports::Cricket),

    }

}