use recommender::Result;
use recommender::dataset::ItemID;
use recommender::dataset::ItemRecord;
use recommender::dataset::Iterator;
use recommender::dataset::PairRecord;
use recommender::dataset::UserID;
use recommender::dataset::UserRecord;
use recommender::dataset;
use std::path::PathBuf;

use parser::Config;
use parser::Parser;

pub struct Dataset {
    path: PathBuf,
    config: Config,
}

impl Dataset {
    pub fn new<T>(path: T, config: Config) -> Dataset
    where
        T: Into<PathBuf>,
    {
        Dataset {
            path: path.into(),
            config: config,
        }
    }
}

impl dataset::Dataset for Dataset {
    type Pairs = Box<Iterator<Item = PairRecord>>;
    type Users = Box<Iterator<Item = UserRecord>>;
    type Items = Box<Iterator<Item = ItemRecord>>;

    #[inline]
    fn pairs(&self) -> Result<Self::Pairs> {
        Ok(Box::new(Parser::from_path(&self.path, self.config)?))
    }

    #[inline]
    fn users(&self, id: ItemID) -> Result<Self::Users> {
        Ok(Box::new(
            Parser::from_path(&self.path, self.config)?
                .filter(move |&((_, item_id), _): &PairRecord| item_id == id)
                .map(|((user_id, _), rating)| (user_id, rating)),
        ))
    }

    #[inline]
    fn items(&self, id: UserID) -> Result<Self::Items> {
        Ok(Box::new(
            Parser::from_path(&self.path, self.config)?
                .filter(move |&((user_id, _), _): &PairRecord| user_id == id)
                .map(|((_, item_id), rating)| (item_id, rating)),
        ))
    }
}
