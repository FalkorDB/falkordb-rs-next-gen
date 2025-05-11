use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hasher},
};

pub struct AggregateIter<I, K, V, F, G>
where
    I: Iterator<Item = V>,
    K: std::hash::Hash,
    F: Fn(&V) -> K,
    G: Fn(V, V) -> V,
{
    pub iter: I,
    pub key_fn: F,
    pub default_value: V,
    pub agg_fn: G,
    pub cache: HashMap<u64, (K, V)>,
    pub finished: bool,
}

impl<I, K, V, F, G> Iterator for AggregateIter<I, K, V, F, G>
where
    I: Iterator<Item = V>,
    K: std::hash::Hash + Clone,
    F: Fn(&V) -> K,
    G: Fn(V, V) -> V,
    V: Clone,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.finished {
            for item in self.iter.by_ref() {
                let key = (self.key_fn)(&item);
                let mut hasher = DefaultHasher::new();
                key.hash(&mut hasher);

                self.cache
                    .entry(hasher.finish())
                    .and_modify(|v| v.1 = (self.agg_fn)(item.clone(), v.1.clone()))
                    .or_insert_with(|| {
                        (key, (self.agg_fn)(item.clone(), self.default_value.clone()))
                    });
            }

            self.finished = true;
        }
        match self.cache.keys().next().copied() {
            Some(key) => self.cache.remove_entry(&key).map(|(_, v)| {
                let (key, value) = v;
                (key, value)
            }),
            None => None,
        }
    }
}
