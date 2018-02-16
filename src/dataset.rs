use std::ops::DerefMut;

use Result;

pub type User = u64;
pub type Item = u64;
pub type Rating = f64;

pub type Pair = (User, Item);

pub type PairRating = (Pair, Rating);
pub type UserRating = (User, Rating);
pub type ItemRating = (Item, Rating);

pub trait Dataset {
    type Pairs: Reader<Item = PairRating>;
    type Users: Reader<Item = UserRating>;
    type Items: Reader<Item = ItemRating>;

    fn pairs(self) -> Result<Self::Pairs>;
    fn users(self, Item) -> Result<Self::Users>;
    fn items(self, User) -> Result<Self::Items>;
}

pub trait Reader {
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
