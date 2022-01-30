# cache: Cache Trait Library Crate
Bart Massey 2022 (version 0.1.0)

This crate provides a `Cache` trait that can be used to
provide a common interface for cache implementations.  The
associated `cache-tests` crate provides some generic cache
tests.

## Examples

```rust
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

This crate is made available under the "MIT
license". Please see the file `LICENSE` in this distribution
for license terms.

Thanks to the `cargo-readme` crate for generation of this
`README`.
