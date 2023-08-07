use crate::component::difficulty::RollStatus;
use crate::component::roll::Dice;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DifficultyClass {
    dc: u64,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DifficultyClassResult {
    /// Whether the roll was a success or not.
    pub success: bool,
    /// The DC (Difficulty Class) that was rolled against.
    pub dc: u64,
    /// The selected roll based on the roll status and number of rolls
    pub roll: u64,
    /// The roll status that was used.
    pub roll_status: RollStatus,
    /// The rolls that were made.
    pub rolls_made: Vec<u64>,
}

impl DifficultyClass {
    pub fn new(dc: u64) -> Self {
        Self { dc }
    }

    pub fn roll(&self, dice: Dice) -> DifficultyClassResult {
        self.check(dice.roll(), RollStatus::Normal)
    }

    pub fn roll_with_advantage(&self, dice: Dice) -> DifficultyClassResult {
        self.roll_twice(dice, RollStatus::Advantage)
    }

    pub fn roll_with_disadvantage(&self, dice: Dice) -> DifficultyClassResult {
        self.roll_twice(dice, RollStatus::Disadvantage)
    }

    pub fn roll_many(&self, dice: Dice, amount: u8, status: RollStatus) -> DifficultyClassResult {
        let mut rolls: Vec<u64> = vec![];

        for _ in 0..amount {
            rolls.push(dice.roll());
        }

        self.check_multiple(rolls, status)
    }

    fn roll_twice(&self, dice: Dice, status: RollStatus) -> DifficultyClassResult {
        self.check_multiple(vec![dice.roll(), dice.roll()], status)
    }

    fn check(&self, roll: u64, status: RollStatus) -> DifficultyClassResult {
        self.check_multiple(vec![roll], status)
    }

    fn check_multiple(&self, rolls: Vec<u64>, status: RollStatus) -> DifficultyClassResult {
        let selected = match status {
            RollStatus::Advantage | RollStatus::Normal => rolls.iter().max().unwrap(),
            RollStatus::Disadvantage => rolls.iter().min().unwrap(),
        };

        DifficultyClassResult {
            success: self.is_success(*selected),
            dc: self.dc,
            roll: *selected,
            roll_status: status.clone(),
            rolls_made: rolls.clone(),
        }
    }

    fn is_success(&self, roll: u64) -> bool {
        roll >= self.dc
    }
}

#[cfg(test)]
mod tests {
    use crate::component::difficulty::{DifficultyClass, RollStatus};
    use crate::component::roll::Dice;

    #[test]
    fn test_dc_roll_success() {
        let always_sucess = DifficultyClass::new(1);
        let dice = Dice::group(1, 20);

        let result = always_sucess.roll(dice);

        assert!(result.success);
        assert_eq!(result.dc, 1);
        assert_eq!(result.roll_status, RollStatus::Normal);
        assert_eq!(result.rolls_made.len(), 1);
    }

    #[test]
    fn test_dc_roll_fail() {
        let always_fail = DifficultyClass::new(21);
        let dice = Dice::group(1, 20);

        let result = always_fail.roll(dice);

        assert!(!result.success);
        assert_eq!(result.dc, 21);
        assert_eq!(result.roll_status, RollStatus::Normal);
        assert_eq!(result.rolls_made.len(), 1);
    }

    #[test]
    fn test_advantage() {
        let dc = DifficultyClass::new(10);
        let dice = Dice::group(1, 20);

        let result = dc.roll_with_advantage(dice);

        assert_eq!(result.dc, 10);
        assert_eq!(result.roll_status, RollStatus::Advantage);
        assert_eq!(result.rolls_made.len(), 2);
    }

    #[test]
    fn test_disadvantage() {
        let dc = DifficultyClass::new(10);
        let dice = Dice::group(1, 20);

        let result = dc.roll_with_disadvantage(dice);

        assert_eq!(result.dc, 10);
        assert_eq!(result.roll_status, RollStatus::Disadvantage);
        assert_eq!(result.rolls_made.len(), 2);
    }
}
