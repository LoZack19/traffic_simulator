mod policy;
mod traffic;

use std::sync::{Arc, RwLock};

use policy::{AllGoIn, Probability, RandomEarlyDetection, Threshold};
use traffic::Traffic;

fn main() {
    let traffic = Traffic::new(100, 300);

    let _all_go_in = AllGoIn;
    let _threshold: Threshold = traffic.define_threshold_policy(10);
    let red: RandomEarlyDetection =
        traffic.define_red_policy(20.0..30.0, 0.02, Probability::try_from(0.8).unwrap());

    traffic.simulate(Arc::new(RwLock::new(red)));
}
