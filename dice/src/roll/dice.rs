use crate::roll::die::Die;
use rand::Rng;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub struct Dice {
    dice: Vec<Die>,
    rng: rand::rngs::ThreadRng,
}

impl Dice {
    pub fn group(amount: u8, sides: u8) -> Self {
        let mut dice: Vec<Die> = vec![];

        (1..=amount).for_each(|_| dice.push(Die::new(sides)));

        Self {
            dice,
            rng: rand::thread_rng(),
        }
    }

    pub fn new(dice: Vec<Die>) -> Self {
        Self {
            dice,
            rng: rand::thread_rng(),
        }
    }

    pub fn collection(&self) -> Vec<Die> {
        self.dice.clone()
    }

    pub fn dice_count(&self) -> usize {
        self.dice.len()
    }

    pub fn is_empty(&self) -> bool {
        self.dice.len() == 0
    }

    pub fn is_mixed(&self) -> bool {
        if self.is_empty() {
            return false;
        }

        let collection = self.collection();
        let first = collection.first().unwrap().sides();

        !self.dice.iter().all(|die| first == die.sides())
    }

    pub fn range(&self) -> RangeInclusive<u64> {
        let collection = self.collection();
        let min = collection
            .iter()
            .map(|die| die.range().min().unwrap())
            .sum::<u8>() as u64;
        let max = collection
            .iter()
            .map(|die| die.range().max().unwrap())
            .sum::<u8>() as u64;

        min..=max
    }

    /// Rolls all die in the collection and returns the sum.
    pub fn roll(&self) -> u64 {
        let mut added: u64 = 0;
        let collection = self.collection();

        collection
            .iter()
            .for_each(|die| added += self.roll_single_die(die) as u64);

        added
    }

    /// A less expensive way to roll the die in the collection.
    ///
    /// This collects the possible range of all die in the collection
    /// and then rolls a random number within that range
    ///
    /// This operation is cheaper than rolling each die individually
    /// and summing the results, as it only calls rand once.
    pub fn roll_cheaply(&self) -> u64 {
        let range = self.range();

        self.rng.clone().gen_range(range)
    }

    /// Rolls all die in the collection and returns the individual values.
    pub fn roll_individually(&self) -> Vec<u8> {
        let mut v = vec![];
        let collection = self.collection();

        collection
            .iter()
            .for_each(|die| v.push(self.roll_single_die(die)));

        v
    }

    /// Rolls all die in the collection and returns the lowest value.
    /// Useful for cases where you might roll with disadvantage
    pub fn roll_min(&self) -> u8 {
        let collection = self.collection();

        collection
            .iter()
            .map(|die| self.roll_single_die(die))
            .min()
            .unwrap()
    }

    /// Rolls all die in the collection and returns the highest value.
    /// Useful for cases where you might roll with advantage
    pub fn roll_max(&self) -> u8 {
        let collection = self.collection();

        collection
            .iter()
            .map(|die| self.roll_single_die(die))
            .max()
            .unwrap()
    }

    fn roll_single_die(&self, die: &Die) -> u8 {
        self.rng.clone().gen_range(die.range())
    }
}

impl Display for Dice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "0d0")
        } else if self.is_mixed() {
            let mut sides = HashMap::new();
            let collection = self.collection();

            collection.iter().for_each(|die| {
                let s = die.sides();

                if sides.contains_key(&s) {
                    let count = sides.get(&s).unwrap() + 1;
                    sides.insert(s, count);
                } else {
                    sides.insert(s, 1u8);
                }
            });

            let mut strings: Vec<String> = vec![];
            sides.iter().for_each(|(s, c)| {
                strings.push(format!("{}d{}", c, s));
            });
            strings.sort_by(|a, b| {
                let a_val = a.split_once('d').unwrap().1.parse::<u8>().unwrap();
                let b_val = b.split_once('d').unwrap().1.parse::<u8>().unwrap();

                b_val.cmp(&a_val)
            });

            write!(f, "{}", strings.join(", "))
        } else {
            let len = self.dice_count();
            let sides = self.dice.first().unwrap().sides();

            write!(f, "{}d{}", len, sides)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::roll::dice::Dice;
    use crate::roll::die::Die;

    #[test]
    fn test_range() {
        let dice = Dice::group(2, 6);
        let range = dice.range();

        assert_eq!(range, 2..=12);
    }

    #[test]
    fn test_roll() {
        let dice = Dice::group(2, 6);
        let roll = dice.roll();

        assert!((2..=12).contains(&roll));
    }

    #[test]
    fn test_group_to_string() {
        let dice = Dice::group(2, 6);
        let string = dice.to_string();

        assert_eq!(string, "2d6");
    }

    #[test]
    fn test_mixed_group_to_string() {
        let dice = Dice::new(vec![Die::new(20), Die::new(6), Die::new(6), Die::new(8)]);
        let string = dice.to_string();

        assert_eq!(string, "1d20, 1d8, 2d6");
    }
}
