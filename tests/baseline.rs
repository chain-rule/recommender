extern crate recommender;

use recommender::Baseline;

use dataset::Dataset;
use reader::Config;

mod dataset;
mod reader;

const RATING_PATH: &'static str = "tests/fixtures/ratings.dat";

macro_rules! ok(($result:expr) => ($result.unwrap()));

#[test]
fn new() {
    let dataset = Dataset::new(RATING_PATH, Config::new().delimiter("::"));
    let _ = ok!(Baseline::new(dataset));
}
