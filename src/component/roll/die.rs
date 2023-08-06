use rand;
use rand::Rng;
use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub struct Die {
    sides: u8,
    rng: Option<rand::rngs::ThreadRng>,
}

impl Die {
    pub fn without_rng(sides: u8) -> Self {
        Self { sides, rng: None }
    }

    pub fn new(sides: u8) -> Self {
        Self {
            sides,
            rng: Some(rand::thread_rng()),
        }
    }

    pub fn sides(&self) -> u8 {
        self.sides
    }

    pub fn range(&self) -> RangeInclusive<u8> {
        1..=self.sides
    }

    pub fn roll(&self) -> u8 {
        match self.rng.clone() {
            None => self.roll_with_rng(rand::thread_rng()),
            Some(rng) => self.roll_with_rng(rng),
        }
    }

    pub fn roll_with_rng(&self, mut rng: rand::rngs::ThreadRng) -> u8 {
        rng.gen_range(self.range())
    }
}

impl Display for Die {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "1d{}", self.sides)
    }
}
