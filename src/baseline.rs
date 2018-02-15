use std::collections::HashMap;

use Result;
use dataset::Dataset;
use dataset::ItemID;
use dataset::Iterator;
use dataset::Rating;
use dataset::UserID;

pub struct Baseline {
    pub global_bias: Rating,
    pub user_biases: HashMap<UserID, Rating>,
    pub item_biases: HashMap<ItemID, Rating>,
}

struct Mean {
    sum: Rating,
    count: usize,
}

impl Baseline {
    pub fn new<T>(dataset: T) -> Result<Self>
    where
        T: Dataset,
    {
        let mut global_mean = Mean::new();
        let mut user_means: HashMap<UserID, Mean> = HashMap::new();
        let mut item_means: HashMap<ItemID, Mean> = HashMap::new();
        let mut iterator = dataset.iter_pairs()?;
        while let Some(((user_id, item_id), rating)) = iterator.next()? {
            global_mean.consume(rating);
            user_means
                .entry(user_id)
                .or_insert_with(|| Mean::new())
                .consume(rating);
            item_means
                .entry(item_id)
                .or_insert_with(|| Mean::new())
                .consume(rating);
        }
        Ok(Baseline {
            global_bias: global_mean.compute(),
            user_biases: user_means
                .iter()
                .map(|(&id, mean)| (id, mean.compute()))
                .collect(),
            item_biases: item_means
                .iter()
                .map(|(&id, mean)| (id, mean.compute()))
                .collect(),
        })
    }
}

impl Mean {
    #[inline]
    fn new() -> Self {
        Mean { sum: 0.0, count: 0 }
    }

    #[inline]
    fn consume(&mut self, value: Rating) {
        self.sum += value;
        self.count += 1;
    }

    #[inline]
    fn compute(&self) -> Rating {
        return self.sum / self.count as Rating;
    }
}
