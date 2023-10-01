use overworld_5e as o5e;

fn main() {
    println!("Select a race!");
    let race = o5e::race::Race::Human;

    println!("You have chosen to become a {}", race.to_string());
}
