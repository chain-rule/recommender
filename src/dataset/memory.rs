use Result;
use dataset::Item;
use dataset::ItemDataset;
use dataset::ItemRating;
use dataset::Iterator;
use dataset::PairDataset;
use dataset::PairRating;
use dataset::Reader;
use dataset::User;
use dataset::UserDataset;
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

impl<'l> PairDataset for &'l Memory {
    type Reader = Iterator<::std::iter::Cloned<::std::slice::Iter<'l, PairRating>>>;

    fn pairs(self) -> Result<Self::Reader> {
        Ok(Iterator::from(self.data.iter().cloned()))
    }
}

impl<'l> UserDataset for &'l Memory {
    type Reader = Box<Reader<Item = UserRating> + 'l>;

    fn users(self, item: Item) -> Result<Self::Reader> {
        Ok(Box::new(Iterator::from(
            self.data
                .iter()
                .filter(move |&&((_, other), _)| other == item)
                .map(|&((user, _), rating)| (user, rating)),
        )))
    }
}

impl<'l> ItemDataset for &'l Memory {
    type Reader = Box<Reader<Item = ItemRating> + 'l>;

    fn items(self, user: User) -> Result<Self::Reader> {
        Ok(Box::new(Iterator::from(
            self.data
                .iter()
                .filter(move |&&((other, _), _)| other == user)
                .map(|&((_, item), rating)| (item, rating)),
        )))
    }
}
