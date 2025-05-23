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

pub struct LazyReplace<I, F>
where
    I: Iterator,
    F: FnOnce() -> I,
{
    iter: Option<I>,
    replacement: Option<F>,
    yielded: bool, // Tracks whether any item has been yielded
}

impl<I, F> LazyReplace<I, F>
where
    I: Iterator,
    F: FnOnce() -> I,
{
    pub const fn new(
        iter: I,
        replacement: F,
    ) -> Self {
        Self {
            iter: Some(iter),
            replacement: Some(replacement),
            yielded: false,
        }
    }
}

impl<I, F> Iterator for LazyReplace<I, F>
where
    I: Iterator,
    F: FnOnce() -> I,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut iter) = self.iter {
            if let Some(item) = iter.next() {
                self.yielded = true; // Mark that an item has been yielded
                return Some(item);
            }
        }

        if !self.yielded {
            if let Some(replacement) = self.replacement.take() {
                self.iter = Some(replacement());
                return self.iter.as_mut().unwrap().next();
            }
        }

        None
    }
}

pub trait TryMap {
    fn try_map<T, E, F>(
        self,
        func: F,
    ) -> impl Iterator<Item = Result<T, E>>
    where
        Self: Iterator<Item = Result<T, E>>,
        F: Fn(T) -> Result<T, E>;
}

impl<I> TryMap for I
where
    I: Iterator,
{
    fn try_map<T, E, F>(
        self,
        func: F,
    ) -> impl Iterator<Item = Result<T, E>>
    where
        Self: Iterator<Item = Result<T, E>>,
        F: Fn(T) -> Result<T, E>,
    {
        self.map(move |x| match x {
            Ok(x) => func(x),
            Err(e) => Err(e),
        })
    }
}

pub trait TryFlatMap {
    fn try_flat_map<T, E, F, FE, I>(
        self,
        func: F,
        err: FE,
    ) -> impl Iterator<Item = Result<T, E>>
    where
        Self: Iterator<Item = Result<T, E>>,
        F: Fn(T) -> I,
        FE: Fn(E) -> I,
        I: Iterator<Item = Result<T, E>>;
}

impl<I> TryFlatMap for I
where
    I: Iterator,
{
    fn try_flat_map<T, E, F, FE, J>(
        self,
        func: F,
        err: FE,
    ) -> impl Iterator<Item = Result<T, E>>
    where
        Self: Iterator<Item = Result<T, E>>,
        F: Fn(T) -> J,
        FE: Fn(E) -> J,
        J: Iterator<Item = Result<T, E>>,
    {
        self.flat_map(move |x| match x {
            Ok(x) => func(x),
            Err(e) => err(e),
        })
    }
}
