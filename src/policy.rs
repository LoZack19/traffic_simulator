use std::sync::{Arc, RwLock};

use crate::traffic::Queue;

pub trait Policy
where
    Self: Send,
{
    fn allow(&self) -> bool;
}

pub struct AllGoIn;

impl Policy for AllGoIn {
    fn allow(&self) -> bool {
        true
    }
}

pub struct Threshold {
    queue: Arc<RwLock<Queue>>,
    threshold: usize,
}

impl Threshold {
    pub fn new(queue: Arc<RwLock<Queue>>, threshold: usize) -> Threshold {
        Threshold { queue, threshold }
    }
}

impl Policy for Threshold {
    fn allow(&self) -> bool {
        self.queue.read().unwrap().len() < self.threshold
    }
}
