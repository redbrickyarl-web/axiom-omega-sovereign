//! Lock-free Ring Buffer for the Silicon Engine

use std::sync::atomic::{AtomicUsize, Ordering};

/// High-performance lock-free ring buffer
pub struct RingBuffer {
    capacity: usize,
    head: AtomicUsize,
    tail: AtomicUsize,
}

impl RingBuffer {
    /// Create new ring buffer
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.next_power_of_two(),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    /// Get capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}