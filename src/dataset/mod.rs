//! Datasets.

use std::ops::DerefMut;

use Result;

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
pub trait PairDataset {
    /// A reader.
    type Reader: Reader<Item = PairRating>;

    /// Create a reader.
    fn pairs(self) -> Result<Self::Reader>;
}

/// A dataset of users and ratings for a given item.
pub trait UserDataset {
    /// A reader.
    type Reader: Reader<Item = UserRating>;

    /// Create a reader.
    fn users(self, Item) -> Result<Self::Reader>;
}

/// A dataset of items and ratings for a given user.
pub trait ItemDataset {
    /// A reader.
    type Reader: Reader<Item = ItemRating>;

    /// Create a reader.
    fn items(self, User) -> Result<Self::Reader>;
}

/// A reader.
pub trait Reader {
    /// A record.
    type Item;

    /// Read the next record.
    fn next(&mut self) -> Result<Option<Self::Item>>;

    /// Filter records.
    #[inline]
    fn filter<T>(self, function: T) -> Filter<Self, T>
    where
        Self: Sized,
    {
        Filter {
            iterator: self,
            function: function,
        }
    }

    /// Map records.
    #[inline]
    fn map<T>(self, function: T) -> Map<Self, T>
    where
        Self: Sized,
    {
        Map {
            iterator: self,
            function: function,
        }
    }
}

/// A filter.
pub struct Filter<T, U> {
    iterator: T,
    function: U,
}

/// A mapping.
pub struct Map<T, U> {
    iterator: T,
    function: U,
}

/// An adaptor for an iterator.
pub struct Iterator<T> {
    iterator: T,
}

impl<T: ?Sized> Reader for Box<T>
where
    T: Reader,
{
    type Item = T::Item;

    #[inline]
    fn next(&mut self) -> Result<Option<Self::Item>> {
        self.deref_mut().next()
    }
}

impl<T, U> Reader for Filter<T, U>
where
    T: Reader,
    U: FnMut(&T::Item) -> bool,
{
    type Item = T::Item;

    fn next(&mut self) -> Result<Option<Self::Item>> {
        while let Some(item) = self.iterator.next()? {
            if (self.function)(&item) {
                return Ok(Some(item));
            }
        }
        Ok(None)
    }
}

impl<T, U, V> Reader for Map<T, U>
where
    T: Reader,
    U: FnMut(T::Item) -> V,
{
    type Item = V;

    fn next(&mut self) -> Result<Option<Self::Item>> {
        if let Some(item) = self.iterator.next()? {
            Ok(Some((self.function)(item)))
        } else {
            Ok(None)
        }
    }
}

impl<T> Reader for Iterator<T>
where
    T: ::std::iter::Iterator,
{
    type Item = T::Item;

    #[inline]
    fn next(&mut self) -> Result<Option<Self::Item>> {
        Ok(self.iterator.next())
    }
}

impl<T> From<T> for Iterator<T>
where
    T: ::std::iter::Iterator,
{
    #[inline]
    fn from(iterator: T) -> Self {
        Iterator { iterator: iterator }
    }
}
