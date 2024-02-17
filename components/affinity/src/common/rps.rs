use crate::Affinity;

#[derive(Debug, PartialEq)]
pub enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl Affinity for RockPaperScissors {
    fn weak_against(&self, other: &Self) -> bool {
        match self {
            RockPaperScissors::Rock => matches!(other, RockPaperScissors::Paper),
            RockPaperScissors::Paper => matches!(other, RockPaperScissors::Scissors),
            RockPaperScissors::Scissors => matches!(other, RockPaperScissors::Rock),
        }
    }

    fn strong_against(&self, other: &Self) -> bool {
        if self.eq(other) {
            return false;
        }

        !self.weak_against(other)
    }

    fn multiplier(&self, other: &Self) -> f32 {
        if self.strong_against(other) {
            1.0
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_weak_against() {
        assert!(RockPaperScissors::Rock.weak_against(&RockPaperScissors::Paper));
        assert!(RockPaperScissors::Paper.weak_against(&RockPaperScissors::Scissors));
        assert!(RockPaperScissors::Scissors.weak_against(&RockPaperScissors::Rock));
    }

    #[test]
    fn test_rock_beats_paper() {
        assert_eq!(
            0.0,
            RockPaperScissors::Rock.calculate(1.0, &RockPaperScissors::Paper),
        );
    }

    #[test]
    fn test_paper_ties_paper() {
        assert_eq!(
            0.0,
            RockPaperScissors::Paper.calculate(1.0, &RockPaperScissors::Paper),
        );
    }

    #[test]
    fn test_scissors_beats_paper() {
        assert_eq!(
            1.0,
            RockPaperScissors::Scissors.calculate(1.0, &RockPaperScissors::Paper),
        );
    }
}
