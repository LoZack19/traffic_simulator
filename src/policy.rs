use std::{
    ops::{Deref, Range},
    sync::{Arc, RwLock},
};

use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::traffic::Queue;

pub trait Policy
where
    Self: Send,
{
    fn allow(&mut self) -> bool;
}

pub struct AllGoIn;

impl Policy for AllGoIn {
    fn allow(&mut self) -> bool {
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
    fn allow(&mut self) -> bool {
        self.queue.read().unwrap().len() < self.threshold
    }
}

#[derive(Debug)]
pub enum ProbabilityError {
    OutOfRange,
}

#[derive(Debug, Copy, Clone)]
pub struct Probability(f64);

impl Probability {
    fn new(num: f64) -> Result<Probability, ProbabilityError> {
        if (0.0..=1.0).contains(&num) {
            Ok(Probability(num))
        } else {
            Err(ProbabilityError::OutOfRange)
        }
    }
}

impl TryFrom<f64> for Probability {
    type Error = ProbabilityError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Probability::new(value)
    }
}

impl Distribution<Probability> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Probability {
        rng.gen_range(0.0..=1.0).try_into().unwrap()
    }
}

impl Deref for Probability {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}

#[derive(Debug)]
pub struct RandomEarlyDetection {
    queue: Arc<RwLock<Queue>>,
    range: Range<f64>,
    average: f64,
    weight: f64,
    max_drop_prob: Probability,
}

impl RandomEarlyDetection {
    pub fn new(
        queue: Arc<RwLock<Queue>>,
        range: Range<f64>,
        weight: f64,
        max_drop_prob: Probability,
    ) -> Result<RandomEarlyDetection, String> {
        if range.start < 0.0 {
            return Err(format!(
                "Range start must be a positive integer, but it was given {}",
                range.start
            ));
        }

        let len = queue.read().unwrap().len();
        Ok(RandomEarlyDetection {
            queue,
            range,
            weight,
            max_drop_prob,
            average: len as f64,
        })
    }

    fn update_average(&mut self) {
        let queue_length = self.queue.read().unwrap().len() as f64;
        let new_average = (1.0 - self.weight) * self.average + self.weight * queue_length;
        self.average = new_average;
    }

    fn drop_probability(&self) -> Probability {
        let d_p = if self.average <= self.range.start {
            0.0
        } else if self.average > self.range.end {
            1.0
        } else {
            *self.max_drop_prob
                * ((self.average - self.range.start) / (self.range.end - self.range.start))
        };

        Probability::new(d_p).unwrap()
    }

    fn update(&mut self) -> Probability {
        self.update_average();
        self.drop_probability()
    }
}

impl Policy for RandomEarlyDetection {
    fn allow(&mut self) -> bool {
        let mut rng = thread_rng();
        let dice = rng.gen::<Probability>();
        let drop_prob = self.update();

        println!(
            "[POLICY]: {{ len: {:.2}, prob: {:.2}, dice: {:.2} }}",
            self.average, *drop_prob, *dice
        );

        *drop_prob <= *dice
    }
}
