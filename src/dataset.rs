use std::ops::DerefMut;

use Result;

pub type UserID = u64;
pub type ItemID = u64;
pub type Rating = f64;

pub type PairID = (UserID, ItemID);

pub type PairRecord = (PairID, Rating);
pub type UserRecord = (UserID, Rating);
pub type ItemRecord = (ItemID, Rating);

pub trait Dataset {
    type Pairs: Iterator<Item = PairRecord>;
    type Users: Iterator<Item = UserRecord>;
    type Items: Iterator<Item = ItemRecord>;

    fn pairs(&self) -> Result<Self::Pairs>;
    fn users(&self, id: ItemID) -> Result<Self::Users>;
    fn items(&self, id: UserID) -> Result<Self::Items>;
}

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Result<Option<Self::Item>>;

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

    #[inline]
    fn pack(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

pub struct Filter<T, U> {
    iterator: T,
    function: U,
}

pub struct Map<T, U> {
    iterator: T,
    function: U,
}

impl<T: ?Sized> Iterator for Box<T>
where
    T: Iterator,
{
    type Item = T::Item;

    #[inline]
    fn next(&mut self) -> Result<Option<Self::Item>> {
        self.deref_mut().next()
    }
}

impl<T, U> Iterator for Filter<T, U>
where
    T: Iterator,
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

impl<T, U, V> Iterator for Map<T, U>
where
    T: Iterator,
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
