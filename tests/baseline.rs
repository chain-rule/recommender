extern crate recommender;

use recommender::Baseline;

use dataset::Disk;
use dataset::Memory;
use parser::Config;

mod dataset;
mod parser;

const RATING_PATH: &'static str = "tests/fixtures/ratings.dat";

macro_rules! assert_equal(
    ($x:expr, $y:expr) => (assert_eq!($x, $y));
    ($x:expr, $y:expr, $epsilon:expr) => (assert!(($x - $y).abs() < $epsilon));
);

macro_rules! ok(
    ($result:expr) => ($result.unwrap());
);

#[test]
fn new() {
    let dataset = Disk::new(RATING_PATH, Config::new().delimiter("::"));
    let dataset = ok!(Memory::new(&dataset));
    let baseline = ok!(Baseline::new(&dataset, 10, 15, 10));
    assert_equal!(baseline.global_bias, 3.581564453029317, 1e-10);
    assert_equal!(baseline.user_biases.len(), 6040);
    assert_equal!(baseline.item_biases.len(), 3706);
    assert_equal!(baseline.user_biases[&42], 0.114951013519927, 1e-10);
    assert_equal!(baseline.item_biases[&42], -0.570961410545486, 1e-10);
}
