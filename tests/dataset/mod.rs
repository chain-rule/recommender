use recommender::Result;
use recommender::dataset::Dataset;
use recommender::dataset::Item;
use recommender::dataset::ItemRating;
use recommender::dataset::Iterator;
use recommender::dataset::PairRating;
use recommender::dataset::Reader;
use recommender::dataset::User;
use recommender::dataset::UserRating;
use std::fs::File;
use std::path::PathBuf;

use parser::Config;
use parser::Parser;

pub struct Disk {
    path: PathBuf,
    config: Config,
}

pub struct Memory {
    data: Vec<PairRating>,
}

impl Disk {
    pub fn new<T>(path: T, config: Config) -> Self
    where
        T: Into<PathBuf>,
    {
        Disk {
            path: path.into(),
            config: config,
        }
    }
}

impl Memory {
    pub fn new<T>(dataset: T) -> Result<Self>
    where
        T: Dataset,
    {
        let mut data = vec![];
        let mut iterator = dataset.pairs()?;
        while let Some(record) = iterator.next()? {
            data.push(record);
        }
        Ok(Memory { data: data })
    }
}

impl<'l> Dataset for &'l Disk {
    type Pairs = Parser<File>;
    type Users = Box<Reader<Item = UserRating>>;
    type Items = Box<Reader<Item = ItemRating>>;

    fn pairs(self) -> Result<Self::Pairs> {
        Parser::from_path(&self.path, self.config)
    }

    fn users(self, item: Item) -> Result<Self::Users> {
        Ok(Parser::from_path(&self.path, self.config)?
            .filter(move |&((_, other), _): &PairRating| other == item)
            .map(|((user, _), rating)| (user, rating))
            .pack())
    }

    fn items(self, user: User) -> Result<Self::Items> {
        Ok(Parser::from_path(&self.path, self.config)?
            .filter(move |&((other, _), _): &PairRating| other == user)
            .map(|((_, item), rating)| (item, rating))
            .pack())
    }
}

impl<'l> Dataset for &'l Memory {
    type Pairs = Iterator<::std::iter::Cloned<::std::slice::Iter<'l, PairRating>>>;
    type Users = Box<Reader<Item = UserRating> + 'l>;
    type Items = Box<Reader<Item = ItemRating> + 'l>;

    fn pairs(self) -> Result<Self::Pairs> {
        Ok(Iterator::from(self.data.iter().cloned()))
    }

    fn users(self, item: Item) -> Result<Self::Users> {
        Ok(Iterator::from(self.data.iter().cloned())
            .filter(move |&((_, other), _): &PairRating| other == item)
            .map(|((user, _), rating)| (user, rating))
            .pack())
    }

    fn items(self, user: User) -> Result<Self::Items> {
        Ok(Iterator::from(self.data.iter().cloned())
            .filter(move |&((other, _), _): &PairRating| other == user)
            .map(|((_, item), rating)| (item, rating))
            .pack())
    }
}
