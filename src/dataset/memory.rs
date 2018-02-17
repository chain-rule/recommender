use Result;
use dataset::Item;
use dataset::ItemRating;
use dataset::Items;
use dataset::PairRating;
use dataset::Pairs;
use dataset::User;
use dataset::UserRating;
use dataset::Users;
use reader::Iterator;
use reader::Reader;

/// A memory-based dataset.
pub struct Memory {
    data: Vec<PairRating>,
}

impl Memory {
    /// Create a dataset given a reader.
    pub fn from_reader<T>(mut reader: T) -> Result<Self>
    where
        T: Reader<Record = PairRating>,
    {
        let mut data = vec![];
        while let Some(record) = reader.next()? {
            data.push(record);
        }
        Ok(Memory { data: data })
    }
}

impl<'l> Pairs for &'l Memory {
    type Reader = Iterator<::std::iter::Cloned<::std::slice::Iter<'l, PairRating>>>;

    fn pairs(self) -> Result<Self::Reader> {
        Ok(Iterator::from(self.data.iter().cloned()))
    }
}

impl<'l> Users for &'l Memory {
    type Reader = Box<Reader<Record = UserRating> + 'l>;

    fn users(self, item: Item) -> Result<Self::Reader> {
        Ok(Box::new(Iterator::from(
            self.data
                .iter()
                .filter(move |&&((_, other), _)| other == item)
                .map(|&((user, _), rating)| (user, rating)),
        )))
    }
}

impl<'l> Items for &'l Memory {
    type Reader = Box<Reader<Record = ItemRating> + 'l>;

    fn items(self, user: User) -> Result<Self::Reader> {
        Ok(Box::new(Iterator::from(
            self.data
                .iter()
                .filter(move |&&((other, _), _)| other == user)
                .map(|&((_, item), rating)| (item, rating)),
        )))
    }
}
