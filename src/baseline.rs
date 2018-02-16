use std::collections::HashMap;

use Result;
use dataset::Dataset;
use dataset::Item;
use dataset::Iterator;
use dataset::Rating;
use dataset::User;

pub struct Baseline {
    pub global_bias: Rating,
    pub user_biases: HashMap<User, Rating>,
    pub item_biases: HashMap<Item, Rating>,
}

struct Mean {
    shrinkage: usize,
    value: Rating,
    count: usize,
}

impl Baseline {
    pub fn new<T>(
        dataset: T,
        n_iterations: usize,
        user_shrinkage: usize,
        item_shrinkage: usize,
    ) -> Result<Self>
    where
        T: Dataset,
    {
        let mut global_mean = Mean::new(0);
        let mut user_means: HashMap<User, Mean> = HashMap::new();
        let mut item_means: HashMap<Item, Mean> = HashMap::new();
        let mut iterator = dataset.pairs()?;
        while let Some(((user, item), rating)) = iterator.next()? {
            global_mean.consume(rating);
            user_means
                .entry(user)
                .or_insert_with(|| Mean::new(user_shrinkage));
            item_means
                .entry(item)
                .or_insert_with(|| Mean::new(item_shrinkage));
        }
        global_mean.finalize();
        for _ in 0..n_iterations {
            item_means.values_mut().for_each(|mean| mean.reset());
            let mut iterator = dataset.pairs()?;
            while let Some(((user, item), rating)) = iterator.next()? {
                item_means
                    .get_mut(&item)
                    .unwrap()
                    .consume(rating - global_mean.value - user_means[&user].value);
            }
            item_means.values_mut().for_each(|mean| mean.finalize());
            user_means.values_mut().for_each(|mean| mean.reset());
            let mut iterator = dataset.pairs()?;
            while let Some(((user, item), rating)) = iterator.next()? {
                user_means
                    .get_mut(&user)
                    .unwrap()
                    .consume(rating - global_mean.value - item_means[&item].value);
            }
            user_means.values_mut().for_each(|mean| mean.finalize());
        }
        Ok(Baseline {
            global_bias: global_mean.value,
            user_biases: user_means
                .iter()
                .map(|(&id, mean)| (id, mean.value))
                .collect(),
            item_biases: item_means
                .iter()
                .map(|(&id, mean)| (id, mean.value))
                .collect(),
        })
    }
}

impl Mean {
    #[inline]
    fn new(shrinkage: usize) -> Self {
        Mean {
            shrinkage: shrinkage,
            value: 0.0,
            count: 0,
        }
    }

    #[inline]
    fn consume(&mut self, value: Rating) {
        self.value += value;
        self.count += 1;
    }

    #[inline]
    fn finalize(&mut self) {
        self.value /= (self.shrinkage + self.count) as Rating;
    }

    #[inline]
    fn reset(&mut self) {
        self.value = 0.0;
        self.count = 0;
    }
}
