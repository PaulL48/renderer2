use crate::material::Material;
use std::collections::HashMap;

pub struct MaterialCache {
    cache: HashMap<u64, Material>,
}

impl MaterialCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: u64, material: Material) -> Option<Material> {
        self.cache.insert(id, material)
    }

    pub fn get(&self, id: &u64) -> Option<&Material> {
        self.cache.get(id)
    }
}
