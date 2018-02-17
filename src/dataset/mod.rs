//! Datasets.

use Result;
use reader::Reader;

pub use self::disk::Disk;
pub use self::memory::Memory;

mod disk;
mod memory;

/// A user.
pub type User = u64;

/// An item.
pub type Item = u64;

/// A rating.
pub type Rating = f64;

/// A user and an item.
pub type Pair = (User, Item);

/// A user, an item, and a rating.
pub type PairRating = (Pair, Rating);

/// A user and a rating.
pub type UserRating = (User, Rating);

/// An item and a rating.
pub type ItemRating = (Item, Rating);

/// A dataset of users, items, and ratings.
pub trait Pairs {
    /// A reader.
    type Reader: Reader<Record = PairRating>;

    /// Create a reader.
    fn pairs(self) -> Result<Self::Reader>;
}

/// A dataset of users and ratings for a given item.
pub trait Users {
    /// A reader.
    type Reader: Reader<Record = UserRating>;

    /// Create a reader.
    fn users(self, Item) -> Result<Self::Reader>;
}

/// A dataset of items and ratings for a given user.
pub trait Items {
    /// A reader.
    type Reader: Reader<Record = ItemRating>;

    /// Create a reader.
    fn items(self, User) -> Result<Self::Reader>;
}
