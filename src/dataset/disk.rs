use std::fs::File;
use std::path::PathBuf;

use Result;
use dataset::Item;
use dataset::ItemDataset;
use dataset::ItemRating;
use dataset::PairDataset;
use dataset::PairRating;
use dataset::Reader;
use dataset::User;
use dataset::UserDataset;
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

impl<'l> PairDataset for &'l Disk {
    type Reader = Text<File>;

    #[inline]
    fn pairs(self) -> Result<Self::Reader> {
        Text::from_path(&self.path, self.config)
    }
}

impl<'l> UserDataset for &'l Disk {
    type Reader = Box<Reader<Record = UserRating>>;

    fn users(self, item: Item) -> Result<Self::Reader> {
        Ok(Box::new(
            Text::from_path(&self.path, self.config)?
                .filter(move |&((_, other), _): &PairRating| other == item)
                .map(|((user, _), rating)| (user, rating)),
        ))
    }
}

impl<'l> ItemDataset for &'l Disk {
    type Reader = Box<Reader<Record = ItemRating>>;

    fn items(self, user: User) -> Result<Self::Reader> {
        Ok(Box::new(
            Text::from_path(&self.path, self.config)?
                .filter(move |&((other, _), _): &PairRating| other == user)
                .map(|((_, item), rating)| (item, rating)),
        ))
    }
}
