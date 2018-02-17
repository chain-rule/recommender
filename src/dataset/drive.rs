use std::fs::File;
use std::path::PathBuf;

use Result;
use dataset::Item;
use dataset::ItemRating;
use dataset::Items;
use dataset::PairRating;
use dataset::Pairs;
use dataset::Reader;
use dataset::User;
use dataset::UserRating;
use dataset::Users;
use parser::Text;
use parser::TextConfig;

/// A drive-based dataset.
pub struct Drive {
    path: PathBuf,
    config: DriveConfig,
}

/// A configuration of a drive-based dataset.
pub type DriveConfig = TextConfig;

impl Drive {
    /// Create a dataset.
    pub fn new<T>(path: T, config: DriveConfig) -> Self
    where
        T: Into<PathBuf>,
    {
        Drive {
            path: path.into(),
            config: config,
        }
    }
}

impl<'l> Pairs for &'l Drive {
    type Reader = Text<File>;

    #[inline]
    fn pairs(self) -> Result<Self::Reader> {
        Text::from_path(&self.path, self.config)
    }
}

impl<'l> Users for &'l Drive {
    type Reader = Box<Reader<Record = UserRating>>;

    fn users(self, item: Item) -> Result<Self::Reader> {
        Ok(Box::new(
            Text::from_path(&self.path, self.config)?
                .filter(move |&((_, other), _): &PairRating| other == item)
                .map(|((user, _), rating)| (user, rating)),
        ))
    }
}

impl<'l> Items for &'l Drive {
    type Reader = Box<Reader<Record = ItemRating>>;

    fn items(self, user: User) -> Result<Self::Reader> {
        Ok(Box::new(
            Text::from_path(&self.path, self.config)?
                .filter(move |&((other, _), _): &PairRating| other == user)
                .map(|((_, item), rating)| (item, rating)),
        ))
    }
}
