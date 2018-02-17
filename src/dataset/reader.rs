use std::ops::DerefMut;

use Result;

/// A reader.
pub trait Reader {
    /// A record.
    type Record;

    /// Read the next record.
    fn next(&mut self) -> Result<Option<Self::Record>>;

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
    type Record = T::Record;

    #[inline]
    fn next(&mut self) -> Result<Option<Self::Record>> {
        self.deref_mut().next()
    }
}

impl<T, U> Reader for Filter<T, U>
where
    T: Reader,
    U: FnMut(&T::Record) -> bool,
{
    type Record = T::Record;

    fn next(&mut self) -> Result<Option<Self::Record>> {
        while let Some(record) = self.iterator.next()? {
            if (self.function)(&record) {
                return Ok(Some(record));
            }
        }
        Ok(None)
    }
}

impl<T, U, V> Reader for Map<T, U>
where
    T: Reader,
    U: FnMut(T::Record) -> V,
{
    type Record = V;

    fn next(&mut self) -> Result<Option<Self::Record>> {
        if let Some(record) = self.iterator.next()? {
            Ok(Some((self.function)(record)))
        } else {
            Ok(None)
        }
    }
}

impl<T> Reader for Iterator<T>
where
    T: ::std::iter::Iterator,
{
    type Record = T::Item;

    #[inline]
    fn next(&mut self) -> Result<Option<Self::Record>> {
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
