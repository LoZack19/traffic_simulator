use std::{
    ops::Range,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, RwLock,
    },
    thread,
    time::Duration,
};

use fixed_vec_deque::FixedVecDeque;
use rand::Rng;

use crate::policy::{Policy, Probability, RandomEarlyDetection, Threshold};

pub type Queue = FixedVecDeque<[i32; 1024]>;

pub struct Traffic {
    queue: Arc<RwLock<Queue>>,
    max_producer_delay: u64,
    max_consumer_delay: u64,
    // policy is instant
}

impl Traffic {
    pub fn new(max_prducer_delay: u64, max_consumer_delay: u64) -> Traffic {
        Traffic {
            queue: Arc::new(RwLock::new(Queue::new())),
            max_producer_delay: max_prducer_delay,
            max_consumer_delay,
        }
    }

    fn queue(&self) -> Arc<RwLock<Queue>> {
        self.queue.clone()
    }

    pub fn define_threshold_policy(&self, threshold: usize) -> Threshold {
        Threshold::new(self.queue(), threshold)
    }

    pub fn define_red_policy(
        &self,
        range: Range<f64>,
        weight: f64,
        max_drop_prob: Probability,
    ) -> RandomEarlyDetection {
        RandomEarlyDetection::new(self.queue.clone(), range, weight, max_drop_prob).unwrap()
    }

    fn random_traffic(tx: Sender<i32>, max_ms: u64) {
        let mut rng = rand::thread_rng();

        loop {
            let delay: Duration = Duration::from_millis(rng.gen_range(0..max_ms));
            let packet: i32 = rng.gen_range(0..100);

            thread::sleep(delay);

            println!("[PRODUCER]: Sending: {packet}");
            tx.send(packet).unwrap();
        }
    }

    fn traffic_manager(
        rx: Receiver<i32>,
        queue: Arc<RwLock<Queue>>,
        policy: Arc<RwLock<impl Policy>>,
    ) {
        for received in rx {
            if policy.write().unwrap().allow() {
                println!("[POLICY]: Allowed: {received}");
                *queue.write().unwrap().push_back() = received;
            } else {
                println!("[POLICY]: Discarded: {received}")
            }
            println!("[POLICY]: Queue: {:?}", queue.read().unwrap());
        }
    }

    fn traffic_consumer(queue: Arc<RwLock<Queue>>, max_ms: u64) {
        let mut rng = rand::thread_rng();

        loop {
            let delay: Duration = Duration::from_millis(rng.gen_range(0..max_ms));
            thread::sleep(delay);
            if let Some(value) = queue.write().unwrap().pop_front() {
                println!("[CONSUMER]: Consumed {value}")
            }
        }
    }

    pub fn simulate(self, policy: Arc<RwLock<(impl Policy + 'static + Sync)>>) {
        let (tx, rx) = mpsc::channel();
        let admin_queue = Arc::clone(&self.queue);
        let consumer_queue = Arc::clone(&self.queue);

        let producer = thread::spawn(move || Self::random_traffic(tx, self.max_producer_delay));
        let admin = thread::spawn(|| Self::traffic_manager(rx, admin_queue, policy));
        let consumer =
            thread::spawn(move || Self::traffic_consumer(consumer_queue, self.max_consumer_delay));

        producer.join().unwrap();
        admin.join().unwrap();
        consumer.join().unwrap();
    }
}
