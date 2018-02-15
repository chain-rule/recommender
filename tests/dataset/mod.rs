use recommender::Result;
use recommender::dataset::{ItemID, ItemRecord, PairRecord, UserID, UserRecord};
use recommender::dataset::{self, Iterator};
use std::path::PathBuf;

use reader::{Config, Reader};

pub struct Dataset {
    path: PathBuf,
    config: Config,
}

impl Dataset {
    pub fn new<T>(path: T, config: Config) -> Dataset
    where
        T: Into<PathBuf>,
    {
        Dataset { path: path.into(), config: config }
    }
}

impl dataset::Dataset for Dataset {
    type PairIterator = Box<Iterator<Item = PairRecord>>;
    type UserIterator = Box<Iterator<Item = UserRecord>>;
    type ItemIterator = Box<Iterator<Item = ItemRecord>>;

    #[inline]
    fn iter_pairs(&self) -> Result<Self::PairIterator> {
        Ok(Box::new(Reader::from_path(&self.path, self.config)?))
    }

    #[inline]
    fn iter_users(&self, id: ItemID) -> Result<Self::UserIterator> {
        Ok(Box::new(
            Reader::from_path(&self.path, self.config)?
                .filter(move |&((_, item_id), _): &PairRecord| item_id == id)
                .map(|((user_id, _), rating)| (user_id, rating)),
        ))
    }

    #[inline]
    fn iter_items(&self, id: UserID) -> Result<Self::ItemIterator> {
        Ok(Box::new(
            Reader::from_path(&self.path, self.config)?
                .filter(move |&((user_id, _), _): &PairRecord| user_id == id)
                .map(|((_, item_id), rating)| (item_id, rating)),
        ))
    }
}
