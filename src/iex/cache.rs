//

use super::client::Client;
use super::reference::Reference;

use cached::proc_macro::cached;
use cached::Cached;

use log::error;
use std::{cmp::Eq, collections::HashMap, hash::Hash, time::Instant};

/// Enum used for defining the status of time-cached values
enum Status {
    NotFound,
    Found,
    Expired,
}

/// Cache store bound by time
///
/// Values are timestamped when inserted and are
/// evicted if expired at time of retrieval.
///
/// Note: This cache is in-memory only
pub struct TimedCache<K, V> {
    store: HashMap<K, (Instant, V)>,
    seconds: u64,
    hits: u64,
    misses: u64,
    initial_capacity: Option<usize>,
}

impl<K: Hash + Eq, V> TimedCache<K, V> {
    #[allow(dead_code)]
    /// Creates a new `TimedCache` with a specified lifespan
    pub fn with_lifespan(seconds: u64) -> TimedCache<K, V> {
        TimedCache {
            store: Self::new_store(None),
            seconds,
            hits: 0,
            misses: 0,
            initial_capacity: None,
        }
    }

    #[allow(dead_code)]
    /// Creates a new `TimedCache` with a specified lifespan and
    /// cache-store with the specified pre-allocated capacity
    pub fn with_lifespan_and_capacity(
        seconds: u64,
        size: usize,
    ) -> TimedCache<K, V> {
        TimedCache {
            store: Self::new_store(Some(size)),
            seconds,
            hits: 0,
            misses: 0,
            initial_capacity: Some(size),
        }
    }

    fn new_store(capacity: Option<usize>) -> HashMap<K, (Instant, V)> {
        capacity.map_or_else(HashMap::new, HashMap::with_capacity)
    }
}

impl<K: Hash + Eq, V> Cached<K, V> for TimedCache<K, V> {
    fn cache_get(&mut self, key: &K) -> Option<&V> {
        let status = {
            let val = self.store.get(key);
            if let Some(&(instant, _)) = val {
                if instant.elapsed().as_secs() < self.seconds {
                    Status::Found
                } else {
                    Status::Expired
                }
            } else {
                Status::NotFound
            }
        };
        match status {
            Status::NotFound => {
                self.misses += 1;
                None
            }
            Status::Found => {
                self.hits += 1;
                self.store.get(key).map(|stamped| &stamped.1)
            }
            Status::Expired => {
                self.misses += 1;
                self.store.remove(key).unwrap();
                None
            }
        }
    }

    fn cache_get_mut(&mut self, key: &K) -> Option<&mut V> {
        let status = {
            let val = self.store.get(key);
            if let Some(&(instant, _)) = val {
                if instant.elapsed().as_secs() < self.seconds {
                    Status::Found
                } else {
                    Status::Expired
                }
            } else {
                Status::NotFound
            }
        };
        match status {
            Status::NotFound => {
                self.misses += 1;
                None
            }
            Status::Found => {
                self.hits += 1;
                self.store.get_mut(key).map(|stamped| &mut stamped.1)
            }
            Status::Expired => {
                self.misses += 1;
                self.store.remove(key).unwrap();
                None
            }
        }
    }
    fn cache_set(&mut self, key: K, val: V) {
        let stamped = (Instant::now(), val);
        self.store.insert(key, stamped);
    }
    fn cache_remove(&mut self, k: &K) -> Option<V> {
        self.store.remove(k).map(|(_, v)| v)
    }
    fn cache_clear(&mut self) {
        self.store.clear();
    }
    fn cache_reset(&mut self) {
        self.store = Self::new_store(self.initial_capacity);
    }
    fn cache_size(&self) -> usize {
        self.store.len()
    }
    fn cache_hits(&self) -> Option<u64> {
        Some(self.hits)
    }
    fn cache_misses(&self) -> Option<u64> {
        Some(self.misses)
    }
    fn cache_lifespan(&self) -> Option<u64> {
        Some(self.seconds)
    }
}

trait Lifespan {
    fn cache_set_lifespan(&mut self, seconds: u64);
}

impl<K: Hash + Eq, V> Lifespan for TimedCache<K, V> {
    fn cache_set_lifespan(&mut self, seconds: u64) {
        self.seconds = seconds;
    }
}

#[cached(
    type = "TimedCache<u8, Option<Vec<String>>>",
    create = "{ TimedCache::with_lifespan(0) }",
    option = true,
    key = "u8",
    convert = r#"{ 0 }"#
)]
pub async fn symbols(client: &Client) -> Option<Vec<String>> {
    match client.symbols().await {
        Ok(s) => Some(s.iter().map(|x| x.symbol.clone()).collect()),
        Err(why) => {
            error!("Could not fetch symbols: {:?}", why);
            None
        }
    }
}

pub async fn set_symbols_lifetime(seconds: u64) {
    SYMBOLS.lock().await.cache_set_lifespan(seconds);
}
