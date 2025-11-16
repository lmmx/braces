/// Simple ordered map that preserves insertion order
#[derive(Debug)]
pub struct OrderedMap<K, V> {
    entries: Vec<(K, V)>,
}

impl<K: PartialEq, V> OrderedMap<K, V> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.entries.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(pos) = self.entries.iter().position(|(k, _)| k == &key) {
            Some(std::mem::replace(&mut self.entries[pos].1, value))
        } else {
            self.entries.push((key, value));
            None
        }
    }

    /// Forward + reverse iteration support
    pub fn iter(&self) -> std::slice::Iter<'_, (K, V)> {
        self.entries.iter()
    }

    /// Forward + reverse iteration support over values
    pub fn values(&self) -> impl DoubleEndedIterator<Item = &V> {
        self.entries.iter().map(|(_, v)| v)
    }

    #[allow(dead_code)]
    /// Like IndexMap: iterate keys
    pub fn keys(&self) -> impl DoubleEndedIterator<Item = &K> {
        self.entries.iter().map(|(k, _)| k)
    }

    #[allow(dead_code)]
    /// Optional: index-style access if needed
    pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {
        self.entries.get(index).map(|(k, v)| (k, v))
    }
}
