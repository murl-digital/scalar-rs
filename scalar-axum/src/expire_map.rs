use std::hash::Hash;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    time::{Duration, Instant},
};

struct HeapValue<K> {
    instant: Instant,
    key: K,
}
impl<K> PartialOrd for HeapValue<K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<K> Ord for HeapValue<K> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.instant.cmp(&other.instant).reverse()
    }
}
impl<K> PartialEq for HeapValue<K> {
    fn eq(&self, other: &Self) -> bool {
        self.instant.eq(&other.instant)
    }
}
impl<K> Eq for HeapValue<K> {}

pub struct ExpiringHashMap<K, V> {
    hash_map: HashMap<K, V>,
    heap: BinaryHeap<HeapValue<K>>,
    duration: Duration,
}

impl<K: Eq + Hash + Clone, V> ExpiringHashMap<K, V> {
    #[must_use]
    pub fn new(duration: Duration) -> Self {
        Self {
            hash_map: HashMap::new(),
            heap: BinaryHeap::new(),
            duration,
        }
    }

    pub fn insert(&mut self, key: K, v: V) -> Option<V> {
        let now = Instant::now();
        self.heap.retain(|k| k.key != key);
        self.heap.push(HeapValue {
            instant: now,
            key: key.clone(),
        });
        self.cleanup();
        self.hash_map.insert(key, v)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.heap.retain(|k| k.key != *key);
        let result = self.hash_map.remove(key);
        self.cleanup();
        result
    }

    /// Cleans up expired entries in this [`ExpiringHashMap<K, V>`].
    pub fn cleanup(&mut self) {
        let now = Instant::now();

        while let Some(HeapValue { instant, key }) = self.heap.pop() {
            // when checked_duration_since returns none, that means
            // that the instant is in the future.
            // since the binary heap is ordering with the earliest instants first
            // (because of the reverse ordering in heapvalue)
            // most likely all instants after this are also in the future
            // so we short circuit out now
            // if this isn't the case (becase of some monotonicity bug)
            // that just means entries won't be evicted when they're supposed to,
            // which is most likely fine :)
            if now
                .checked_duration_since(instant)
                .is_none_or(|d| d < self.duration)
            {
                return;
            }

            self.hash_map.remove(&key);
        }
    }
}
