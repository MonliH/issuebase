use chrono::{Duration, NaiveDateTime, Utc};
use rocket::tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    hash::Hash,
    io::Write,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheEntry<T> {
    pub value: T,
    pub time: NaiveDateTime,
}

#[derive(Debug)]
pub struct Cache<K, V> {
    pub cache: RwLock<HashMap<K, CacheEntry<V>>>,
}

impl<K, V> Cache<K, V>
where
    for<'a> K: Eq + Hash + Serialize + Deserialize<'a>,
    for<'a> V: Clone + Serialize + Deserialize<'a>,
{
    pub async fn write_to_file(self, key: &str) {
        let c = self.cache.into_inner();
        let encoded = bincode::serialize(&c).unwrap();
        let mut f = File::create(Self::format_key(key)).expect("failed to create a file");
        f.write_all(&encoded).expect("failed to write data");
    }

    pub fn read_from_file(key: &str) -> Option<Self> {
        let bytes: Vec<u8> = fs::read(Self::format_key(key)).ok()?;
        let map: HashMap<K, CacheEntry<V>> = bincode::deserialize(&bytes).unwrap();
        Some(Self {
            cache: RwLock::new(map),
        })
    }
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
        }
    }

    fn format_key(key: &str) -> String {
        format!("{}.cache", key)
    }

    /// Get a certian value from the cache.
    /// Clones the value if found.
    pub async fn get(&self, key: &K, regen_time: Duration) -> Option<V> {
        let map = self.cache.read().await;
        let entry = map.get(key)?;
        if entry.regenerate(regen_time) {
            None
        } else {
            Some(entry.value.clone())
        }
    }

    pub async fn insert(&self, key: K, value: V) {
        let mut map = self.cache.write().await;
        map.insert(
            key,
            CacheEntry {
                value,
                time: Utc::now().naive_utc(),
            },
        );
    }
}

impl<T> CacheEntry<T> {
    // I know this is quite naive for a cache, but it's not worth using
    // something like memcached for such a small project.
    pub fn regenerate(&self, regen_time: Duration) -> bool {
        let delta = Utc::now().naive_utc() - self.time;
        delta > regen_time
    }
}
