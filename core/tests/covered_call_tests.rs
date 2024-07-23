extern crate cqf;

use cqf::core::models::black_scholes::BlackScholesModel;
use cqf::core::strategies::covered_call::CoveredCall;
use cqf::core::strategies::OptionStrategy;

#[test]
fn test_covered_call() {
    let model = BlackScholesModel;
    let s = 100.0;
    let k = 100.0;
    let r = 0.05;
    let sigma = 0.2;
    let t = 1.0;
    let covered_call = CoveredCall::new(&model, s, k, r, sigma, t);
    let price = covered_call.price();
    println!("Covered Call Option Price: {:.2}", price);
    assert!(price > 0.0 && price < 100.0);
}