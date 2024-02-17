use overworld_affinity::{common::elements::Element, Affinity};
use rand::Rng;
use std::fmt::{Display, Formatter};

struct Player {
    name: String,
    hp: i32,
    element: Element,
}

fn main() {
    let mut player1 = random_choice(1);
    let mut player2 = random_choice(2);
    let damage = 10;
    let mut turn = 1;

    loop {
        println!("{} | {}", player1, player2);
        if player1.hp <= 0 {
            println!("\nPlayer 2 wins!");
            break;
        } else if player2.hp <= 0 {
            println!("\nPlayer 1 wins!");
            break;
        }
        println!("\nTurn {} start!\n", turn);
        std::thread::sleep(std::time::Duration::from_millis(500));

        println!("Turn {}: {} => {}", turn, player1, player2);
        let multiplier = player1.element.calculate(1.0, &player2.element);
        println!("  => with a {} multiplier", multiplier);
        println!("  => did {} damage!", damage as f64 * multiplier);
        player2.hp -= (damage as f64 * multiplier) as i32;

        println!("Turn {}: {} => {}", turn, player2, player1);
        let multiplier = player2.element.calculate(1.0, &player1.element);
        println!("  => with a {} multiplier", multiplier);
        println!("  => did {} damage!", damage as f64 * multiplier);
        player1.hp -= (damage as f64 * multiplier) as i32;
        turn += 1;
    }
}

fn random_choice(id: u8) -> Player {
    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen();

    let element = match n1 % 25 {
        0..=4 => Element::Fire,
        5..=9 => Element::Water,
        10..=14 => Element::Ice,
        15..=19 => Element::Grass,
        20..=21 => Element::Electric,
        22..=23 => Element::Ground,
        _ => Element::Flying,
    };

    Player {
        name: format!("Player {}", id),
        hp: 100,
        element,
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (HP: {}, Element: {:?})",
            self.name, self.hp, self.element
        )
    }
}
