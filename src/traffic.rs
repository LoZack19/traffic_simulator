use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, RwLock,
    },
    thread,
    time::Duration,
};

use fixed_vec_deque::FixedVecDeque;
use rand::Rng;

use crate::policy::{Policy, Threshold};

pub type Queue = FixedVecDeque<[i32; 1024]>;

pub struct Traffic {
    queue: Arc<RwLock<Queue>>,
}

impl Traffic {
    pub fn new() -> Traffic {
        Traffic {
            queue: Arc::new(RwLock::new(Queue::new())),
        }
    }

    fn queue(&self) -> Arc<RwLock<Queue>> {
        self.queue.clone()
    }

    pub fn define_threshold_policy(&self, threshold: usize) -> Threshold {
        Threshold::new(self.queue(), threshold)
    }

    fn random_traffic(tx: Sender<i32>) {
        let mut rng = rand::thread_rng();

        loop {
            let delay: Duration = Duration::from_millis(rng.gen_range(0..1000));
            let packet: i32 = rng.gen_range(0..100);

            thread::sleep(delay);

            println!("Sending: {}", packet);
            tx.send(packet).unwrap();
        }
    }

    fn traffic_manager(rx: Receiver<i32>, queue: Arc<RwLock<Queue>>, policy: impl Policy) {
        for received in rx {
            if policy.allow() {
                println!("Allowed: {received}");
                *queue.write().unwrap().push_back() = received;
            } else {
                println!("Discarded: {received}")
            }
            println!("{:?}", queue.read().unwrap());
        }
    }

    pub fn simulate(&mut self, policy: impl Policy + 'static) {
        let (tx, rx) = mpsc::channel();
        let consumer_queue = Arc::clone(&self.queue);

        let producer = thread::spawn(|| Self::random_traffic(tx));
        let consumer = thread::spawn(|| Self::traffic_manager(rx, consumer_queue, policy));

        producer.join().unwrap();
        consumer.join().unwrap();
    }
}
