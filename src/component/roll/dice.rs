use crate::component::roll::die::Die;
use rand::Rng;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Dice {
    dice: Vec<Die>,
    rng: rand::rngs::ThreadRng,
}

impl Dice {
    pub fn amount(amount: u8, sides: u8) -> Self {
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

        self.dice.iter().all(|die| first == die.sides())
    }

    pub fn roll(&mut self) -> u64 {
        let mut added: u64 = 0;
        let collection = self.collection();

        collection
            .iter()
            .for_each(|die| added += self.roll_single_die(die) as u64);

        added
    }

    pub fn roll_individually(&mut self) -> Vec<u8> {
        let mut v = vec![];
        let collection = self.collection();

        collection
            .iter()
            .for_each(|die| v.push(self.roll_single_die(die)));

        v
    }

    pub fn roll_min(&mut self) -> u8 {
        let collection = self.collection();

        collection
            .iter()
            .map(|die| self.roll_single_die(die))
            .min()
            .unwrap()
    }

    pub fn roll_max(&mut self) -> u8 {
        let collection = self.collection();

        collection
            .iter()
            .map(|die| self.roll_single_die(die))
            .max()
            .unwrap()
    }

    fn roll_single_die(&mut self, die: &Die) -> u8 {
        self.rng.gen_range(die.range())
    }
}

impl Display for Dice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "0d0")
        } else if self.is_mixed() {
            let mut strings: Vec<String> = vec![];
            let collection = self.collection();

            collection
                .iter()
                .for_each(|die| strings.push(die.to_string()));

            write!(f, "{}", strings.join(", "))
        } else {
            let len = self.dice_count();
            let sides = self.dice.first().unwrap();

            write!(f, "{}d{}", len, sides)
        }
    }
}
