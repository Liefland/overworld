/// A traditional experience-based XP Tracker
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExperienceTracker {
    experience_points: u64,
    session_gains: Vec<u64>,
    session_losses: Vec<u64>,
    tracking: bool,
}

impl ExperienceTracker {
    pub fn tracked(experience_points: u64, gains: Vec<u64>, losses: Vec<u64>) -> Self {
        Self {
            experience_points,
            session_gains: gains,
            session_losses: losses,
            tracking: true,
        }
    }

    pub fn untracked(experience_points: u64) -> Self {
        Self {
            experience_points,
            session_gains: vec![],
            session_losses: vec![],
            tracking: false,
        }
    }

    pub fn from(experience_points: u64) -> Self {
        Self {
            experience_points,
            session_gains: Vec::new(),
            session_losses: Vec::new(),
            tracking: true,
        }
    }

    pub fn new() -> Self {
        Self::from(0)
    }

    pub fn get(&self) -> u64 {
        self.experience_points
    }

    pub fn add(&mut self, experience_points: u64) {
        if self.tracking {
            self.session_gains.push(experience_points);
        }
        self.experience_points += experience_points;
    }

    pub fn remove(&mut self, experience_points: u64) {
        if experience_points > self.experience_points {
            if self.tracking {
                self.session_losses.push(self.experience_points);
            }
            self.experience_points = 0;
            return;
        }

        if self.tracking {
            self.session_losses.push(experience_points);
        }
        self.experience_points -= experience_points;
    }

    pub fn session_history(&self) -> (Vec<u64>, Vec<u64>) {
        (self.session_gains.clone(), self.session_losses.clone())
    }

    pub fn is_tracking(&self) -> bool {
        self.tracking
    }

    pub fn set(&mut self, experience_points: u64) {
        self.experience_points = experience_points;
    }
}

impl Default for ExperienceTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::component::progression::experience::ExperienceTracker;

    #[test]
    fn test_experience_tracker() {
        let mut experience_tracker = ExperienceTracker::new();

        assert_eq!(experience_tracker.get(), 0);

        experience_tracker.add(100);
        assert_eq!(experience_tracker.get(), 100);

        experience_tracker.remove(50);
        assert_eq!(experience_tracker.get(), 50);

        experience_tracker.set(0);
        assert_eq!(experience_tracker.get(), 0);
    }

    #[test]
    fn test_experience_tracker_negative() {
        let mut experience_tracker = ExperienceTracker::new();

        assert_eq!(experience_tracker.get(), 0);
        experience_tracker.remove(50);
        assert_eq!(experience_tracker.get(), 0);
    }

    #[test]
    fn test_untracked() {
        let mut experience_tracker = ExperienceTracker::untracked(0);

        experience_tracker.add(50);
        experience_tracker.remove(50);

        let (gains, losses) = experience_tracker.session_history();
        assert!(gains.is_empty());
        assert!(losses.is_empty());
    }

    #[test]
    fn test_tracked() {
        let mut experience_tracker = ExperienceTracker::tracked(0, vec![], vec![]);

        experience_tracker.add(50);
        experience_tracker.remove(50);

        let (gains, losses) = experience_tracker.session_history();
        assert!(!gains.is_empty());
        assert!(!losses.is_empty());
    }
}
