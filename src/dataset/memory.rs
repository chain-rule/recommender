use Result;
use dataset::Dataset;
use dataset::Item;
use dataset::ItemRating;
use dataset::Iterator;
use dataset::PairRating;
use dataset::Reader;
use dataset::User;
use dataset::UserRating;

/// A database that reads records from memory.
pub struct Memory {
    data: Vec<PairRating>,
}

impl Memory {
    /// Create a database given a reader.
    pub fn from_reader<T>(mut reader: T) -> Result<Self>
    where
        T: Reader<Item = PairRating>,
    {
        let mut data = vec![];
        while let Some(record) = reader.next()? {
            data.push(record);
        }
        Ok(Memory { data: data })
    }
}

impl<'l> Dataset for &'l Memory {
    type Pairs = Iterator<::std::iter::Cloned<::std::slice::Iter<'l, PairRating>>>;
    type Users = Box<Reader<Item = UserRating> + 'l>;
    type Items = Box<Reader<Item = ItemRating> + 'l>;

    fn pairs(self) -> Result<Self::Pairs> {
        Ok(Iterator::from(self.data.iter().cloned()))
    }

    fn users(self, item: Item) -> Result<Self::Users> {
        Ok(Box::new(Iterator::from(
            self.data
                .iter()
                .filter(move |&&((_, other), _)| other == item)
                .map(|&((user, _), rating)| (user, rating)),
        )))
    }

    fn items(self, user: User) -> Result<Self::Items> {
        Ok(Box::new(Iterator::from(
            self.data
                .iter()
                .filter(move |&&((other, _), _)| other == user)
                .map(|&((_, item), rating)| (item, rating)),
        )))
    }
}
