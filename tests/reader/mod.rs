use recommender::Result;
use recommender::dataset::{Iterator, PairRecord};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Reader<T> {
    reader: T,
    buffer: String,
    config: Config,
}

#[derive(Clone, Copy)]
pub struct Config {
    delimiter: &'static str,
}

impl<T> Reader<T> {
    #[inline]
    pub fn new(reader: T, config: Config) -> Self {
        Reader {
            reader: reader,
            buffer: String::new(),
            config: config,
        }
    }
}

impl Reader<BufReader<File>> {
    pub fn from_path<T>(path: T, config: Config) -> Result<Self>
    where
        T: AsRef<Path>,
    {
        Ok(Reader::new(BufReader::new(File::open(path)?), config))
    }
}

impl<T> Iterator for Reader<T>
where
    T: BufRead,
{
    type Item = PairRecord;

    fn next(&mut self) -> Result<Option<Self::Item>> {
        self.buffer.clear();
        match self.reader.read_line(&mut self.buffer)? {
            0 => return Ok(None),
            _ => {}
        }
        let mut fields = self.buffer.trim().split(self.config.delimiter);
        let user_id = match fields.next() {
            None => return Ok(None),
            Some(field) => field.parse()?,
        };
        let item_id = match fields.next() {
            None => return Ok(None),
            Some(field) => field.parse()?,
        };
        let rating = match fields.next() {
            None => return Ok(None),
            Some(field) => field.parse()?,
        };
        Ok(Some(((user_id, item_id), rating)))
    }
}

impl Config {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn delimiter(mut self, value: &'static str) -> Self {
        self.delimiter = value;
        self
    }
}

impl Default for Config {
    #[inline]
    fn default() -> Self {
        Config { delimiter: "," }
    }
}
