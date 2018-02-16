use recommender::Result;
use recommender::dataset::Dataset;
use recommender::dataset::Item;
use recommender::dataset::ItemRating;
use recommender::dataset::Iterator;
use recommender::dataset::PairRating;
use recommender::dataset::User;
use recommender::dataset::UserRating;
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
    type Pairs = Box<Iterator<Item = PairRating>>;
    type Users = Box<Iterator<Item = UserRating>>;
    type Items = Box<Iterator<Item = ItemRating>>;

    #[inline]
    fn pairs(&self) -> Result<Self::Pairs> {
        Ok(Parser::from_path(&self.path, self.config)?.pack())
    }

    #[inline]
    fn users(&self, item: Item) -> Result<Self::Users> {
        Ok(Parser::from_path(&self.path, self.config)?
            .filter(move |&((_, other), _): &PairRating| other == item)
            .map(|((user, _), rating)| (user, rating))
            .pack())
    }

    #[inline]
    fn items(&self, user: User) -> Result<Self::Items> {
        Ok(Parser::from_path(&self.path, self.config)?
            .filter(move |&((other, _), _): &PairRating| other == user)
            .map(|((_, item), rating)| (item, rating))
            .pack())
    }
}
