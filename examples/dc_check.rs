use incr_lib::component::difficulty::DifficultyClass;
use incr_lib::component::roll::Dice;

fn main() {
    let d20 = Dice::group(1, 20);
    let dc10 = DifficultyClass::new(10);

    let result = dc10.roll_with_advantage(d20);

    let rolls: Vec<String> = result.rolls_made.iter().map(|r| r.to_string()).collect();

    println!(
        "DC {} | You rolled a {}, which is a {}. (advantage, rolled twice: {})",
        result.dc,
        result.roll,
        if result.success { "success" } else { "failure" },
        rolls.join(" | "),
    );
}
