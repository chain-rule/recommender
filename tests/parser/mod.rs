use recommender::Result;
use recommender::dataset::Iterator;
use recommender::dataset::PairRating;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub struct Parser<T> {
    reader: T,
    buffer: String,
    config: Config,
}

#[derive(Clone, Copy)]
pub struct Config {
    delimiter: &'static str,
}

impl<T> Parser<T> {
    #[inline]
    pub fn new(reader: T, config: Config) -> Self {
        Parser {
            reader: reader,
            buffer: String::new(),
            config: config,
        }
    }
}

impl Parser<BufReader<File>> {
    pub fn from_path<T>(path: T, config: Config) -> Result<Self>
    where
        T: AsRef<Path>,
    {
        Ok(Parser::new(BufReader::new(File::open(path)?), config))
    }
}

impl<T> Iterator for Parser<T>
where
    T: BufRead,
{
    type Item = PairRating;

    fn next(&mut self) -> Result<Option<Self::Item>> {
        self.buffer.clear();
        match self.reader.read_line(&mut self.buffer)? {
            0 => return Ok(None),
            _ => {}
        }
        let mut fields = self.buffer.trim().split(self.config.delimiter);
        let user = match fields.next() {
            None => return Ok(None),
            Some(field) => field.parse()?,
        };
        let item = match fields.next() {
            None => return Ok(None),
            Some(field) => field.parse()?,
        };
        let rating = match fields.next() {
            None => return Ok(None),
            Some(field) => field.parse()?,
        };
        Ok(Some(((user, item), rating)))
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
