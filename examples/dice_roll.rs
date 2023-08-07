use incr_lib::component::roll::{Dice, Die};

fn main() {
    {
        let d6 = Die::new(6);
        let result = d6.roll();
        println!(
            "You rolled a {} for {} (range: {:?})",
            d6,
            result,
            d6.range()
        );
    }

    {
        let dice_3d6 = Dice::group(3, 6);
        let result = dice_3d6.roll();
        println!(
            "You rolled {} for {} (range: {:?})",
            dice_3d6,
            result,
            dice_3d6.range()
        );
    }

    {
        let dice_1d20_2d6 = Dice::new(vec![Die::new(20), Die::new(6), Die::new(6)]);
        let result = dice_1d20_2d6.roll();
        println!(
            "You rolled {} for {} (range: {:?})",
            dice_1d20_2d6,
            result,
            dice_1d20_2d6.range()
        );
    }
}
