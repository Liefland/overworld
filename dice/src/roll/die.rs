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
        if self.sides == 0 {
            return 0;
        }

        if self.rng.is_none() {
            let mut rng = rand::thread_rng();
            return rng.gen_range(self.range());
        }

        let mut rng = self.rng.clone().unwrap();
        rng.gen_range(self.range())
    }
}

impl Display for Die {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "1d{}", self.sides)
    }
}

#[cfg(test)]
mod tests {
    use crate::roll::Die;

    #[test]
    fn test_range() {
        let d6 = Die::new(6);
        let range = d6.range();

        assert_eq!(range, 1..=6);
    }

    #[test]
    fn test_roll() {
        let d6 = Die::new(6);
        let result = d6.roll();

        assert!((1..=6).contains(&result));
    }

    #[test]
    fn test_to_string() {
        let d6 = Die::new(6);
        let result = d6.to_string();

        assert_eq!(result, "1d6");
    }
}
