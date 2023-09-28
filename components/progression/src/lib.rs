pub mod experience;
// pub mod milestone;

trait LevelUpTable {
    fn name(&self) -> String;

    fn to_level(&self, experience_points: u64) -> u64;

    fn get_previous_milestone(&self, experience_points: u64) -> Option<u64>;
    fn get_current_milestone(&self, experience_points: u64) -> u64;
    fn get_next_milestone(&self, experience_points: u64) -> Option<u64>;

    fn is_maxed(&self, experience_points: u64) -> bool;
}
