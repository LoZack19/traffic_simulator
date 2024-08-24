mod policy;
mod traffic;

use std::sync::{Arc, RwLock};

use policy::{AllGoIn, Probability, RandomEarlyDetection, Threshold};
use traffic::Traffic;

fn main() {
    let mut traffic = Traffic::new();

    let all_go_in = AllGoIn;
    let threshold: Threshold = traffic.define_threshold_policy(10);
    let red: RandomEarlyDetection = traffic.define_red_policy(1..2, 0.02, Probability::from(0.5));

    traffic.simulate(Arc::new(RwLock::new(red)));
}
