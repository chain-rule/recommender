use std::fs::File;
use std::path::PathBuf;

use Result;
use dataset::Dataset;
use dataset::Item;
use dataset::ItemRating;
use dataset::PairRating;
use dataset::Reader;
use dataset::User;
use dataset::UserRating;
use parser::Text;
use parser::TextConfig;

/// A dataset that reads records from disk.
pub struct Disk {
    path: PathBuf,
    config: TextConfig,
}

impl Disk {
    /// Create a dataset.
    pub fn new<T>(path: T, config: TextConfig) -> Self
    where
        T: Into<PathBuf>,
    {
        Disk {
            path: path.into(),
            config: config,
        }
    }
}

impl<'l> Dataset for &'l Disk {
    type Pairs = Text<File>;
    type Users = Box<Reader<Item = UserRating>>;
    type Items = Box<Reader<Item = ItemRating>>;

    fn pairs(self) -> Result<Self::Pairs> {
        Text::from_path(&self.path, self.config)
    }

    fn users(self, item: Item) -> Result<Self::Users> {
        Ok(Box::new(
            Text::from_path(&self.path, self.config)?
                .filter(move |&((_, other), _): &PairRating| other == item)
                .map(|((user, _), rating)| (user, rating)),
        ))
    }

    fn items(self, user: User) -> Result<Self::Items> {
        Ok(Box::new(
            Text::from_path(&self.path, self.config)?
                .filter(move |&((other, _), _): &PairRating| other == user)
                .map(|((_, item), rating)| (item, rating)),
        ))
    }
}
