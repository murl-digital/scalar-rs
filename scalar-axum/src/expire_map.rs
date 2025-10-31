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
    hash_map: HashMap<K, (Instant, V)>,
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
        self.cleanup();
        let now = Instant::now();
        if let Some(prev) = self.hash_map.insert(key.clone(), (now, v)) {
            Some(prev.1)
        } else {
            self.heap.push(HeapValue { instant: now, key });
            None
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.heap.retain(|k| k.key != *key);
        let result = self.hash_map.remove(key);
        self.cleanup();
        result.map(|(_, v)| v)
    }

    /// Cleans up expired entries in this [`ExpiringHashMap<K, V>`].
    ///
    /// # Panics
    ///
    /// This should never panic, but it may if `BinaryHeap::peek` returns `Some` and `BinaryHeap::pop` returns `None`.
    pub fn cleanup(&mut self) {
        let now = Instant::now();

        while let Some(HeapValue { instant, .. }) = self.heap.peek() {
            if now
                .checked_duration_since(*instant)
                .is_none_or(|d| d < self.duration)
            {
                return;
            }

            let key = self.heap.pop().expect("We know it is not empty.").key;

            let real_instant = self.hash_map[&key].0;

            if now
                .checked_duration_since(real_instant)
                .is_none_or(|d| d < self.duration)
            {
                self.heap.push(HeapValue {
                    instant: real_instant,
                    key,
                });
            } else {
                self.hash_map.remove(&key);
            }
        }
    }
}
