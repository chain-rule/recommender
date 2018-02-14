use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

pub struct Reader<T> {
    delimiter: &'static str,
    reader: T,
    buffer: String,
}

impl<T> Reader<T> {
    pub fn new(reader: T) -> Self {
        Reader {
            delimiter: ",",
            reader: reader,
            buffer: String::new(),
        }
    }

    pub fn delimiter(mut self, value: &'static str) -> Self {
        self.delimiter = value;
        return self;
    }
}

impl<T> Reader<T>
where
    T: BufRead,
{
    pub fn next(&mut self, fields: &mut Vec<String>) -> Result<bool> {
        self.buffer.clear();
        if self.reader.read_line(&mut self.buffer)? == 0 {
            return Ok(false);
        }
        let mut count: usize = 0;
        for field in self.buffer.trim().split(self.delimiter) {
            if fields.len() == count {
                fields.push(field.to_string());
            } else {
                fields[count].clear();
                fields[count].push_str(field);
            }
            count += 1;
        }
        fields.truncate(count);
        Ok(true)
    }
}

impl Reader<BufReader<File>> {
    pub fn from_path<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        Ok(Reader::new(BufReader::new(File::open(path)?)))
    }
}
