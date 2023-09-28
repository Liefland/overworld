use std::fmt::{Display, Formatter};
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GameResource {
    pub resource_name: String,
    pub resource_value: AtomicU64,
}

pub trait Resource {
    fn new(name: String, value: u64) -> Self;
    fn name(&self) -> String;
    fn value(&self) -> u64;

    fn add(&mut self, amount: u64) -> u64;
    fn remove(&mut self, amount: u64) -> Result<u64, &str>;
}

impl Resource for GameResource {
    fn new(name: String, value: u64) -> Self {
        Self {
            resource_name: name,
            resource_value: AtomicU64::from(value),
        }
    }

    fn name(&self) -> String {
        self.resource_name.clone()
    }

    fn value(&self) -> u64 {
        self.resource_value.load(Relaxed)
    }

    fn add(&mut self, amount: u64) -> u64 {
        self.resource_value.fetch_add(amount, Relaxed)
    }

    fn remove(&mut self, amount: u64) -> Result<u64, &str> {
        if self.resource_value.load(Relaxed) < amount {
            return Err("Cannot remove more than the current value of the resource.");
        }

        Ok(self.resource_value.fetch_sub(amount, Relaxed))
    }
}

impl Clone for GameResource {
    fn clone(&self) -> Self {
        Self {
            resource_name: self.resource_name.clone(),
            resource_value: AtomicU64::new(self.resource_value.load(Relaxed)),
        }
    }
}

impl Display for GameResource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({})",
            self.resource_name,
            self.resource_value.load(Relaxed)
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::resource::{GameResource, Resource};

    #[test]
    fn test_resource() {
        let mut resource = GameResource::new("Test Resource".to_string(), 100);

        assert_eq!(resource.name(), "Test Resource");
        assert_eq!(resource.value(), 100);

        resource.add(100);
        assert_eq!(resource.value(), 200);

        resource
            .remove(100)
            .expect("test failure - cannot remove 100 from 200?");
        assert_eq!(resource.value(), 100);

        resource
            .remove(100)
            .expect("test failure - cannot remove 100 from 100?");
        assert_eq!(resource.value(), 0);
    }

    #[test]
    #[should_panic]
    fn test_resource_panic() {
        let mut resource = GameResource::new("Test Resource".to_string(), 0);
        resource.remove(100).unwrap();
    }
}
