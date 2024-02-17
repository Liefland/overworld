use overworld_affinity::{common::rps::RockPaperScissors, Affinity};
use rand::Rng;

fn main() {
    let player1 = random_choice();
    let player2 = random_choice();

    println!("Player 1: {:?}", player1);
    println!("Player 2: {:?}", player2);

    if player1.eq(&player2) {
        println!("It's a tie!");
    } else if player1.calculate(1.0, &player2) > 0. {
        println!("Player 1 wins!");
    } else {
        println!("Player 2 wins!");
    }
}

fn random_choice() -> RockPaperScissors {
    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen();

    match n1 % 3 {
        0 => RockPaperScissors::Rock,
        1 => RockPaperScissors::Paper,
        _ => RockPaperScissors::Scissors,
    }
}
