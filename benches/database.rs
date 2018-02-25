#![feature(test)]

extern crate recommender;
extern crate test;

use recommender::dataset::Drive;
use recommender::dataset::DriveConfig;
use recommender::dataset::Memory;
use recommender::dataset::Pairs;
use recommender::reader::Reader;
use test::Bencher;
use test::black_box;

const RATING_PATH: &'static str = "tests/fixtures/ratings.dat";

macro_rules! ok(
    ($result:expr) => ($result.unwrap());
);

#[bench]
fn drive(bencher: &mut Bencher) {
    let dataset = Drive::new(RATING_PATH, DriveConfig::new().delimiter("::"));
    benchmark(&dataset, bencher);
}

#[bench]
fn memory(bencher: &mut Bencher) {
    let dataset = Drive::new(RATING_PATH, DriveConfig::new().delimiter("::"));
    let dataset = ok!(Memory::from_reader(ok!(dataset.pairs())));
    benchmark(&dataset, bencher);
}

fn benchmark<T>(dataset: T, bencher: &mut Bencher)
where
    T: Copy + Pairs,
{
    let mut reader = ok!(dataset.pairs());
    bencher.iter(|| {
        if let Some(record) = ok!(reader.next()) {
            black_box(record);
        } else {
            reader = ok!(dataset.pairs());
        }
    });
}
