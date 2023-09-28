use overworld::difficulty::DifficultyClass;
use overworld::roll::Dice;

fn main() {
    let dc = DifficultyClass::new(10);
    let die = Dice::group(1, 20);
    let roll = dc.roll(die);

    println!("{:?}", roll);

    if roll.success {
        println!("You succeeded!");
    } else {
        println!("You failed!");
    }
}
