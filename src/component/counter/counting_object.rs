use std::ops::Deref;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

/// A thread-safe implementation of a counting object
#[derive(Debug)]
pub struct CountingObject {
    /// The name of the object
    name: String,
    /// How many workers this object has
    counters: AtomicU64,
    /// How much this object increments the counter for each worker
    increments_by: AtomicU64,
    /// Percentage effectiveness these workers are, starts at 100, lower to debuff, increase to buff
    /// This multiplies the increments_by value.
    effectiveness: AtomicU64,
    /// Numeric cost of the first purchase
    initial_cost: AtomicU64,
    /// Percentage of the initial_cost by which the cost of upgrading increases
    buy_cost: AtomicU64,
    /// Percentage of how much is refunded of the original buying price if you sell back a counter
    sell_cost: AtomicU64,
}

impl CountingObject {
    pub fn from(
        name: String,
        counters: AtomicU64,
        increments_by: AtomicU64,
        effectiveness: AtomicU64,
        initial_cost: AtomicU64,
        buy_cost: AtomicU64,
        sell_cost: AtomicU64,
    ) -> Self {
        CountingObject {
            name,
            counters,
            increments_by,
            effectiveness,
            initial_cost,
            buy_cost,
            sell_cost,
        }
    }

    pub fn new(
        name: String,
        increments_by: AtomicU64,
        initial_cost: AtomicU64,
        buy_cost: AtomicU64,
        sell_cost: AtomicU64,
    ) -> Self {
        CountingObject {
            name,
            counters: AtomicU64::new(0),
            increments_by,
            effectiveness: AtomicU64::new(0),
            initial_cost,
            buy_cost,
            sell_cost,
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn workers(&mut self) -> u64 {
        *self.counters.get_mut().deref()
    }

    pub fn refund_cost(&mut self) -> u64 {
        let workers = self.workers() - 1u64;

        self.calc_buy_cost(workers) * self.sell_cost.get_mut().deref() / 100
    }

    pub fn buy_cost(&mut self) -> u64 {
        let workers = self.workers();
        self.calc_buy_cost(workers)
    }

    pub fn can_afford_buy(&mut self, money: u64) -> bool {
        money >= self.buy_cost()
    }

    /// non-threadsafe - one part of a multi-step transaction
    /// These ops should run in an isolated transaction to prevent race conditions
    /// - determine costs
    /// - add to counter
    /// - subtract from currency
    /// returns true if successful, false if not
    pub fn perform_buy(&mut self, money: u64) -> bool {
        if !self.can_afford_buy(money) {
            return false;
        }
        self.counters.fetch_add(1, Ordering::SeqCst);
        true
    }

    /// non-threadsafe - but less important to be threadsafe than buy, as the way this function is
    /// set up there is very little risk of problems.
    /// These ops should run in an isolated transaction to prevent race conditions
    /// - determine costs
    /// - add to counter
    /// - subtract from currency
    /// returns the amount of currency refunded, you need to manually add this to the money pool
    pub fn perform_sell(&mut self) -> u64 {
        self.counters.fetch_sub(1, Ordering::SeqCst);
        self.refund_cost()
    }

    pub fn sum(&mut self) -> u64 {
        ((self.counters.get_mut().deref() * self.increments_by.get_mut().deref())
            * self.effectiveness.get_mut().deref())
            / 100
    }

    fn calc_buy_cost(&mut self, workers: u64) -> u64 {
        self.initial_cost.get_mut().deref() * (workers * self.buy_cost.get_mut().deref()) / 100
    }
}
