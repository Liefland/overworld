use crate::experience::{ExperienceLevelUpTable, ExperienceTracker};
use crate::LevelUpTable;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExperienceSystem {
    tracker: ExperienceTracker,
    levels: ExperienceLevelUpTable,
}

impl ExperienceSystem {
    pub fn simple(experience_points: u64, table: Vec<u64>) -> Self {
        Self::new(
            ExperienceTracker::from(experience_points),
            ExperienceLevelUpTable::new(table),
        )
    }

    pub fn new(tracker: ExperienceTracker, levels: ExperienceLevelUpTable) -> Self {
        Self { tracker, levels }
    }

    pub fn get_level(&self) -> u64 {
        self.levels.to_level(self.tracker.get())
    }

    pub fn get_experience(&self) -> u64 {
        self.tracker.get()
    }

    pub fn get_experience_remaining(&self) -> Option<u64> {
        self.levels
            .get_next_milestone(self.tracker.get())
            .map(|xp| xp - self.tracker.get())
    }

    pub fn get_next_milestone(&self) -> Option<u64> {
        self.levels.get_next_milestone(self.tracker.get())
    }

    pub fn add_experience(&mut self, amount: u64) {
        self.tracker.add(amount);
    }

    pub fn remove_experience(&mut self, amount: u64) {
        self.tracker.remove(amount);
    }
}

#[cfg(test)]
mod tests {
    use crate::experience::experience_system::ExperienceSystem;
    use crate::experience::{ExperienceLevelUpTable, ExperienceTracker};

    fn new_experience_level_up_system() -> ExperienceSystem {
        let tracker = ExperienceTracker::new();
        let levels = ExperienceLevelUpTable::named(String::from("Test"), vec![100, 250, 500]);

        ExperienceSystem { tracker, levels }
    }

    #[test]
    fn test_level_up() {
        let mut system = new_experience_level_up_system();

        assert_eq!(0, system.get_experience());
        assert_eq!(1, system.get_level());

        system.add_experience(100);
        assert_eq!(100, system.get_experience());
        assert_eq!(2, system.get_level());
    }

    #[test]
    fn test_level_down() {
        let mut system = new_experience_level_up_system();

        system.add_experience(100);
        assert_eq!(100, system.get_experience());
        assert_eq!(2, system.get_level());

        system.remove_experience(100);
        assert_eq!(0, system.get_experience());
        assert_eq!(1, system.get_level());
    }

    #[test]
    fn test_experience_remaining() {
        let mut system = new_experience_level_up_system();

        assert_eq!(100, system.get_experience_remaining().unwrap());
        system.add_experience(100);
        assert_eq!(150, system.get_experience_remaining().unwrap());

        system.add_experience(50);
        assert_eq!(100, system.get_experience_remaining().unwrap());
        assert_eq!(250, system.get_next_milestone().unwrap());

        system.add_experience(100);
        assert_eq!(250, system.get_experience_remaining().unwrap());

        system.add_experience(250);
        assert!(system.get_experience_remaining().is_none());
        assert!(system.get_next_milestone().is_none());
    }
}
