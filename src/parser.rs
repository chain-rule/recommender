use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

use Result;
use dataset::PairRating;
use dataset::Reader;

pub struct Parser<T> {
    reader: BufReader<T>,
    buffer: String,
    config: Config,
}

#[derive(Clone, Copy)]
pub struct Config {
    delimiter: &'static str,
}

impl<T> Parser<T>
where
    T: Read,
{
    #[inline]
    pub fn new(reader: T, config: Config) -> Self {
        Parser {
            reader: BufReader::new(reader),
            buffer: String::new(),
            config: config,
        }
    }
}

impl Parser<File> {
    pub fn from_path<T>(path: T, config: Config) -> Result<Self>
    where
        T: AsRef<Path>,
    {
        Ok(Parser::new(File::open(path)?, config))
    }
}

impl<T> Reader for Parser<T>
where
    T: Read,
{
    type Item = PairRating;

    fn next(&mut self) -> Result<Option<Self::Item>> {
        self.buffer.clear();
        match self.reader.read_line(&mut self.buffer)? {
            0 => return Ok(None),
            _ => {}
        }
        let mut fields = self.buffer.split(self.config.delimiter);
        let user = match fields.next() {
            Some(field) => field.parse()?,
            _ => return Ok(None),
        };
        let item = match fields.next() {
            Some(field) => field.parse()?,
            _ => return Ok(None),
        };
        let rating = match fields.next() {
            Some(field) => field.parse()?,
            _ => return Ok(None),
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
