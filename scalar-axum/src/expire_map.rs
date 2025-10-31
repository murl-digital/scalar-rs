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
        match self.hash_map.insert(key.clone(), (now, v)) {
            Some(prev) => Some(prev.1),
            None => {
                self.heap.push(HeapValue { instant: now, key });
                None
            }
        }
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        self.heap.retain(|k| k.key != key);
        let result = self.hash_map.remove(&key);
        self.cleanup();
        result.map(|(_, v)| v)
    }

    pub fn cleanup(&mut self) {
        let deadline = Instant::now() - self.duration;

        while let Some(HeapValue { instant, .. }) = self.heap.peek() {
            if *instant > deadline {
                return;
            }

            let key = self.heap.pop().expect("We know it is not empty.").key;

            let real_instant = self.hash_map[&key].0;

            if real_instant > deadline {
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
