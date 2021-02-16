use crate::record::Record;
use core::ops::Index;
use core::ops::Range;

pub struct MultiMap<K, V> {
    vec: Vec<Record<K, V>>
}

impl<K, V> MultiMap<K, V> {
    pub fn new(vec: Vec<Record<K, V>>) -> Self {
        Self { vec }
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }
}

impl<'m, K: Ord, V> MultiMap<K, V> {
    pub fn find<'k>(&'m self, k: &'k K) -> Find<'m, 'k, K, V> {
        todo!()
    }

    pub fn find_all(&'m self, k: &K) -> FindAll<'m, K, V> {
        let r = self.range(k).unwrap_or(0..0);
        let records = &self.vec[r];
        let index = 0;
        FindAll { records, index }
    }

    pub fn insert(&mut self, k: K, v: V) -> usize {
        todo!()
    }

    pub fn remove(&mut self, index: usize) -> (K, V) {
        self.vec.remove(index).into_pair()
    }

    pub fn remove_all(&mut self, k: &K) -> usize {
        todo!()
    }

    fn first_index(&self, k: &K) -> Result<usize, usize> {
        todo!()
    }

    fn last_index(&self, k: &K) -> Result<usize, usize> {
        todo!()
    }

    fn range(&self, k: &K) -> Option<Range<usize>> {
        todo!()
    }
}

pub struct Find<'m, 'k, K, V> {
    map: &'m MultiMap<K, V>,
    key: &'k K,
    idx: usize,
    phase: u8,
}

impl<'m, 'k, K: Ord, V> Iterator for Find<'m, 'k, K, V> {
    type Item = &'m V;

    fn next(&mut self) -> Option<Self::Item> {
        match self.phase {
            0 => {
                self.idx = self.map.first_index(self.key).ok()?;
                self.phase = 1;
                self.next()
            }
            1 => {
                let r = &self.map.vec[self.idx];
                if self.key == r.key() {
                    self.idx = self.idx + 1;
                    Some(r.value())
                } else {
                    None
                }
            }
            _ => unreachable!()
        }
    }
}

pub struct FindAll<'m, K, V> {
    records: &'m [Record<K, V>],
    index: usize,
}

impl<K, V> FindAll<'_, K, V> {
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }
}

impl<'m, K, V> Iterator for FindAll<'m, K, V> {
    type Item = &'m V;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.records.len() {
            return None;
        }
        let value = self.records[self.index].value();
        self.index = self.index + 1;
        Some(value)
    }
}