use std::collections::HashMap;

#[derive(Debug)]
pub struct OrderedMap<K, V> {
    map: HashMap<K, V>,
    keys: Vec<K>,
}

impl<K, V> OrderedMap<K, V> where K: Eq + std::hash::Hash + Clone {

    pub fn new() -> Self {
        OrderedMap {
            map: HashMap::new(),
            keys: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.map.contains_key(&key) {
            self.map.insert(key.clone(), value)
        } else {
            self.keys.push(key.clone());
            self.map.insert(key, value)
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.keys.iter().filter_map(move |key| {
            let value = self.map.get(key)?;
            Some((key, value))
        })
    }

    pub fn keys(&self) -> &Vec<K> {
        &self.keys
    }
}