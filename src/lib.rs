/*!
This crate provides a `Cache` trait that can be used to
provide a common interface for cache implementations.  The
associated `cache-tests` crate provides some generic cache
tests.

# Examples

```
use cache::Cache;

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Default)]
struct HashMapCache<K, I>(HashMap<K, I>);

impl<K: Hash + Eq, I> HashMapCache<K, I> {
    fn insert(&mut self, key: K, item: I) {
        self.0.insert(key, item);
    }
    fn retrieve(&mut self, key: &K) -> Option<&mut I> {
        self.0.get_mut(&key)
    }
}

impl<K: Hash + Eq, I> Cache<K> for HashMapCache<K, I> {
    type Item = I;

    fn insert(&mut self, key: K, item: I) {
        self.insert(key, item);
    }
    fn retrieve(&mut self, key: &K) -> Option<&mut I> {
        self.retrieve(key)
    }
}

cache_tests::test_fib_cache(Box::new(HashMapCache::default()));
```
*/


/// A generic cache.
pub trait Cache<K> {
    /// Type of item being cached.
    type Item;
    /// Insert an item in the cache at the given key.
    fn insert(&mut self, key: K, item: Self::Item);
    /// Retrieve the item in the cache at the given key, if any.
    fn retrieve(&mut self, key: &K) -> Option<&mut Self::Item>;
    /// Report the capacity limit of the cache. If `None`, the
    /// cache may grow without bound.
    fn capacity(&self) -> Option<usize> {
        None
    }
}
