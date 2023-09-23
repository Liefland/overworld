use crate::component::progression::LevelUpTable;
use std::ops::Index;

/// A traditional Experience-based Level Up Table
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExperienceLevelUpTable {
    name: String,
    milestones: Vec<u64>,
}

impl LevelUpTable for ExperienceLevelUpTable {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn to_level(&self, experience_points: u64) -> u64 {
        match self.seek(experience_points) {
            None => 1u64,
            Some(milestone) => 2 + milestone as u64,
        }
    }

    fn get_previous_milestone(&self, experience_points: u64) -> Option<u64> {
        if *self.milestones.first().unwrap() > experience_points {
            return None;
        }

        let pointer = self.seek(experience_points).unwrap();

        if pointer == 0 {
            return Some(*self.milestones.index(pointer));
        }

        Some(*self.milestones.index(pointer - 1))
    }

    fn get_current_milestone(&self, experience_points: u64) -> u64 {
        let pointer = self.seek(experience_points);

        if pointer.is_none() {
            return 0;
        }

        *self.milestones.index(pointer.unwrap())
    }

    fn get_next_milestone(&self, experience_points: u64) -> Option<u64> {
        if self.is_maxed(experience_points) {
            return None;
        }

        let pointer = self.seek(experience_points);

        if pointer.is_none() {
            return Some(*self.milestones.index(0));
        }

        Some(*self.milestones.index(pointer.unwrap() + 1))
    }

    fn is_maxed(&self, experience_points: u64) -> bool {
        experience_points >= *self.milestones.last().unwrap()
    }
}

impl ExperienceLevelUpTable {
    pub fn named(name: String, milestones: Vec<u64>) -> Self {
        // Ensure sorting
        let mut ms = milestones.clone();
        ms.sort();

        Self {
            name,
            milestones: ms,
        }
    }

    pub fn new(milestones: Vec<u64>) -> Self {
        Self::named(String::from("Default"), milestones)
    }

    fn seek(&self, experience_points: u64) -> Option<usize> {
        let mut pointer = None;

        for (index, milestone) in self.milestones.iter().enumerate() {
            if experience_points >= *milestone {
                pointer = Some(index);
            }
        }

        pointer
    }
}

#[cfg(test)]
mod tests {
    use crate::component::progression::experience::ExperienceLevelUpTable;
    use crate::component::progression::LevelUpTable;

    fn new_experience_level_up_table() -> ExperienceLevelUpTable {
        ExperienceLevelUpTable::named("Test".to_string(), vec![1, 10, 50, 100])
    }

    #[test]
    fn test_basic_sanity() {
        let table = new_experience_level_up_table();

        assert_eq!("Test".to_string(), table.name());

        assert!(!table.is_maxed(0));
        assert!(table.is_maxed(100));
    }

    #[test]
    fn test_index() {
        let table = new_experience_level_up_table();

        assert_eq!(1, table.to_level(0));
        assert_eq!(2, table.to_level(1));
        assert_eq!(3, table.to_level(10));
        assert_eq!(4, table.to_level(50));
        assert_eq!(5, table.to_level(100));
        assert_eq!(5, table.to_level(10_000));
    }

    #[test]
    fn test_current_milestone() {
        let table = new_experience_level_up_table();

        assert_eq!(0, table.get_current_milestone(0));
        assert_eq!(1, table.get_current_milestone(1));
        assert_eq!(1, table.get_current_milestone(2));
    }

    #[test]
    fn test_previous_milestone() {
        let table = new_experience_level_up_table();

        assert!(table.get_previous_milestone(0).is_none());
        assert_eq!(1, table.get_previous_milestone(10).unwrap());
        assert_eq!(10, table.get_previous_milestone(99).unwrap());
        assert_eq!(50, table.get_previous_milestone(100).unwrap());
    }

    #[test]
    fn test_next_milestone() {
        let table = new_experience_level_up_table();

        assert!(table.get_next_milestone(100).is_none());

        assert_eq!(1, table.get_next_milestone(0).unwrap());
        assert_eq!(10, table.get_next_milestone(9).unwrap());
        assert_eq!(50, table.get_next_milestone(10).unwrap());
    }
}
