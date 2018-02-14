extern crate recommender;

mod reader;

use reader::Reader;

const RATING_PATH: &'static str = "tests/fixtures/ratings.dat";

macro_rules! ok(($result:expr) => ($result.unwrap()));

#[test]
fn compute() {
    let mut reader = ok!(Reader::from_path(RATING_PATH)).delimiter("::");
    let mut fields = Vec::new();
    while ok!(reader.next(&mut fields)) {}
}
