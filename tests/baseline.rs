extern crate recommender;

use recommender::baseline::Baseline;
use recommender::dataset::Disk;
use recommender::dataset::Memory;
use recommender::parser::TextConfig;

const RATING_PATH: &'static str = "tests/fixtures/ratings.dat";

macro_rules! assert_equal(
    ($x:expr, $y:expr) => (assert_eq!($x, $y));
    ($x:expr, $y:expr, $epsilon:expr) => (assert!(($x - $y).abs() < $epsilon));
);

macro_rules! ok(
    ($result:expr) => ($result.unwrap());
);

#[test]
fn from_dataset() {
    let dataset = Disk::new(RATING_PATH, TextConfig::new().delimiter("::"));
    let dataset = ok!(Memory::from_dataset(&dataset));
    let baseline = ok!(Baseline::from_dataset(&dataset, 10, 15, 10));
    assert_equal!(baseline.global_bias, 3.581564453029317, 1e-10);
    assert_equal!(baseline.user_biases.len(), 6040);
    assert_equal!(baseline.item_biases.len(), 3706);
    assert_equal!(baseline.user_biases[&42], 0.114951013519927, 1e-10);
    assert_equal!(baseline.item_biases[&42], -0.570961410545486, 1e-10);
}
