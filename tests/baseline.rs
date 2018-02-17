extern crate recommender;

use recommender::baseline::Baseline;
use recommender::dataset::Drive;
use recommender::dataset::DriveConfig;
use recommender::dataset::Memory;
use recommender::dataset::Pairs;

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
    let dataset = Drive::new(RATING_PATH, DriveConfig::new().delimiter("::"));
    let dataset = ok!(Memory::from_reader(ok!(dataset.pairs())));
    let baseline = ok!(Baseline::from_dataset(&dataset, 10, 15, 10));
    assert_equal!(baseline.global_bias, 3.581564453029317, 1e-10);
    assert_equal!(baseline.user_biases.len(), 6040);
    assert_equal!(baseline.item_biases.len(), 3706);
    assert_equal!(baseline.user_biases[&42], 0.114951013519927, 1e-10);
    assert_equal!(baseline.item_biases[&42], -0.570961410545486, 1e-10);
}
