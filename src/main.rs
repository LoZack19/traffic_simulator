mod policy;
mod traffic;

use policy::{AllGoIn, Threshold};
use traffic::Traffic;

fn main() {
    let mut traffic = Traffic::new();

    let all_go_in = AllGoIn;
    let threshold: Threshold = traffic.define_threshold_policy(10);

    traffic.simulate(threshold);
}
