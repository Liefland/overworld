pub trait Affinity {
    /// Check if the affinity is weak to another
    fn weak_against(&self, other: &Self) -> bool;

    /// Check if the affinity is strong to another
    fn strong_against(&self, other: &Self) -> bool {
        !self.weak_against(other)
    }

    /// Default implementation for a multiplier
    fn multiplier(&self, other: &Self) -> f32 {
        if self.weak_against(other) {
            0.5
        } else if self.strong_against(other) {
            2.0
        } else {
            1.0
        }
    }

    /// Calculate the amount based on the affinity
    fn calculate(&self, amount: f64, other: &Self) -> f64 {
        let multiplier = self.multiplier(other);

        amount * multiplier as f64
    }
}
