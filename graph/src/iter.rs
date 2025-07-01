use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hasher},
    iter::once,
};

pub struct AggregateIter<I, K, V, F, G>
where
    I: Iterator<Item = V>,
    K: std::hash::Hash,
    F: Fn(&V) -> K,
    G: Fn(u64, V, V) -> V,
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
    G: Fn(u64, V, V) -> V,
    V: Clone,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.finished {
            for item in self.iter.by_ref() {
                let key = (self.key_fn)(&item);
                let group_key = {
                    let mut h = DefaultHasher::new();
                    key.hash(&mut h);
                    h.finish()
                };

                self.cache
                    .entry(group_key)
                    .and_modify(|v| v.1 = (self.agg_fn)(group_key, item.clone(), v.1.clone()))
                    .or_insert_with(|| {
                        (
                            key,
                            (self.agg_fn)(group_key, item.clone(), self.default_value.clone()),
                        )
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

pub trait Aggregate {
    fn aggregate<K, V, F, G>(
        self,
        key_fn: F,
        default_value: V,
        agg_fn: G,
        cache: HashMap<u64, (K, V)>,
    ) -> AggregateIter<Box<Self>, K, V, F, G>
    where
        Self: Iterator<Item = V>,
        K: std::hash::Hash + Clone,
        F: Fn(&V) -> K,
        G: Fn(u64, V, V) -> V,
        V: Clone;
}

impl<I> Aggregate for I
where
    I: Iterator,
{
    fn aggregate<K, V, F, G>(
        self,
        key_fn: F,
        default_value: V,
        agg_fn: G,
        cache: HashMap<u64, (K, V)>,
    ) -> AggregateIter<Box<I>, K, V, F, G>
    where
        Self: Iterator<Item = V>,
        K: std::hash::Hash + Clone,
        F: Fn(&V) -> K,
        G: Fn(u64, V, V) -> V,
        V: Clone,
    {
        AggregateIter {
            iter: Box::new(self),
            key_fn,
            default_value,
            agg_fn,
            cache,
            finished: false,
        }
    }
}

pub struct LazyReplaceIter<I, F>
where
    I: Iterator,
    F: FnOnce() -> I,
{
    iter: Option<I>,
    replacement: Option<F>,
    yielded: bool, // Tracks whether any item has been yielded
}

impl<I, F> LazyReplaceIter<I, F>
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

impl<I, F> Iterator for LazyReplaceIter<I, F>
where
    I: Iterator,
    F: FnOnce() -> I,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut iter) = self.iter
            && let Some(item) = iter.next()
        {
            self.yielded = true; // Mark that an item has been yielded
            return Some(item);
        }

        if !self.yielded
            && let Some(replacement) = self.replacement.take()
        {
            self.iter = Some(replacement());
            return self.iter.as_mut().unwrap().next();
        }

        None
    }
}

pub trait LazyReplace
where
    Self: Iterator,
{
    fn lazy_replace<F>(
        self,
        replacement: F,
    ) -> LazyReplaceIter<Self, F>
    where
        Self: Sized,
        F: FnOnce() -> Self;
}

impl<I> LazyReplace for I
where
    I: Iterator,
{
    fn lazy_replace<F: FnOnce() -> I>(
        self,
        replacement: F,
    ) -> LazyReplaceIter<Self, F> {
        LazyReplaceIter::new(self, replacement)
    }
}

pub struct TryMapIter<I, T, E, F>
where
    I: Iterator<Item = Result<T, E>>,
    F: Fn(T) -> Result<T, E>,
{
    iter: I,
    func: F,
    is_error: bool,
}

impl<I, T, E, F> Iterator for TryMapIter<I, T, E, F>
where
    I: Iterator<Item = Result<T, E>>,
    F: Fn(T) -> Result<T, E>,
{
    type Item = Result<T, E>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_error {
            return None; // If an error has already occurred, stop iterating
        }

        match self.iter.next() {
            Some(Ok(value)) => Some((self.func)(value)),
            Some(Err(err)) => {
                self.is_error = true; // Mark that an error has occurred
                Some(Err(err))
            }
            None => {
                None // No more items to iterate
            }
        }
    }
}

pub trait TryMap {
    fn try_map<T1, T2, E, F>(
        self,
        func: F,
    ) -> impl Iterator<Item = Result<T2, E>>
    where
        Self: Iterator<Item = Result<T1, E>>,
        F: Fn(T1) -> Result<T2, E>;
}

impl<I> TryMap for I
where
    I: Iterator,
{
    fn try_map<T1, T2, E, F>(
        self,
        func: F,
    ) -> impl Iterator<Item = Result<T2, E>>
    where
        Self: Iterator<Item = Result<T1, E>>,
        F: Fn(T1) -> Result<T2, E>,
    {
        self.map(move |x| x.and_then(&func))
    }
}

pub struct TryFlatMapIter<I, T, E, F, J>
where
    I: Iterator<Item = Result<T, E>>,
    F: Fn(T) -> J,
    J: Iterator<Item = Result<T, E>>,
{
    iter: I,
    func: F,
    is_error: bool,
    flat_mapped: Option<J>,
}

impl<I, T, E, F, J> Iterator for TryFlatMapIter<I, T, E, F, J>
where
    I: Iterator<Item = Result<T, E>>,
    F: Fn(T) -> J,
    J: Iterator<Item = Result<T, E>>,
{
    type Item = Result<T, E>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_error {
            return None; // If an error has already occurred, stop iterating
        }

        match self
            .flat_mapped
            .as_mut()
            .map_or_else(|| None, std::iter::Iterator::next)
        {
            Some(Ok(flat_value)) => return Some(Ok(flat_value)),
            Some(Err(err)) => {
                self.is_error = true; // Mark that an error has occurred
                return Some(Err(err));
            }
            None => {}
        }

        match self.iter.next() {
            Some(Ok(value)) => {
                self.flat_mapped = Some((self.func)(value));
                match self
                    .flat_mapped
                    .as_mut()
                    .map_or_else(|| None, std::iter::Iterator::next)
                {
                    Some(Ok(flat_value)) => Some(Ok(flat_value)),
                    Some(Err(err)) => {
                        self.is_error = true; // Mark that an error has occurred
                        Some(Err(err))
                    }
                    None => self.next(),
                }
            }
            Some(Err(err)) => {
                self.is_error = true; // Mark that an error has occurred
                Some(Err(err))
            }
            None => None, // No more items to iterate
        }
    }
}

pub trait TryFlatMap {
    fn try_flat_map<'a, T: 'a, E: 'a, F, I>(
        self,
        func: F,
    ) -> impl Iterator<Item = Result<T, E>>
    where
        Self: Iterator<Item = Result<T, E>>,
        F: Fn(T) -> Result<I, E>,
        I: Iterator<Item = Result<T, E>> + 'a;
}

impl<I> TryFlatMap for I
where
    I: Iterator,
{
    fn try_flat_map<'a, T: 'a, E: 'a, F, J>(
        self,
        func: F,
    ) -> impl Iterator<Item = Result<T, E>>
    where
        Self: Iterator<Item = Result<T, E>>,
        F: Fn(T) -> Result<J, E>,
        J: Iterator<Item = Result<T, E>> + 'a,
    {
        self.take_while(Result::is_ok).flat_map(move |x| match x {
            Ok(x) => match func(x) {
                Ok(iter) => Box::new(iter) as Box<dyn Iterator<Item = Result<T, E>>>,
                Err(err) => Box::new(once(Err(err))) as Box<dyn Iterator<Item = Result<T, E>>>,
            },
            Err(err) => Box::new(once(Err(err))) as Box<dyn Iterator<Item = Result<T, E>>>,
        })
    }
}
