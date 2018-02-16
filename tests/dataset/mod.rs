use recommender::Result;
use recommender::dataset::Dataset;
use recommender::dataset::ItemID;
use recommender::dataset::ItemRecord;
use recommender::dataset::Iterator;
use recommender::dataset::PairRecord;
use recommender::dataset::UserID;
use recommender::dataset::UserRecord;
use std::path::PathBuf;

use parser::Config;
use parser::Parser;

pub struct Disk {
    path: PathBuf,
    config: Config,
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

impl Dataset for Disk {
    type Pairs = Box<Iterator<Item = PairRecord>>;
    type Users = Box<Iterator<Item = UserRecord>>;
    type Items = Box<Iterator<Item = ItemRecord>>;

    #[inline]
    fn pairs(&self) -> Result<Self::Pairs> {
        Ok(Parser::from_path(&self.path, self.config)?.pack())
    }

    #[inline]
    fn users(&self, id: ItemID) -> Result<Self::Users> {
        Ok(Parser::from_path(&self.path, self.config)?
            .filter(move |&((_, item_id), _): &PairRecord| item_id == id)
            .map(|((user_id, _), rating)| (user_id, rating))
            .pack())
    }

    #[inline]
    fn items(&self, id: UserID) -> Result<Self::Items> {
        Ok(Parser::from_path(&self.path, self.config)?
            .filter(move |&((user_id, _), _): &PairRecord| user_id == id)
            .map(|((_, item_id), rating)| (item_id, rating))
            .pack())
    }
}
