use std::collections::HashMap;

use thiserror::Error;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use super::Resolver;

// #[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct MapResolver {
    data: HashMap<String, String>,
}

// #[cfg_attr(feature = "wasm", wasm_bindgen)]
impl MapResolver {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, program: String) {
        self.data.insert(name, program);
    }
}

impl Resolver for MapResolver {
    type Error = MapResolverError;

    fn resolve(&mut self, name: &str) -> Result<String, Self::Error> {
        self.data
            .get(name)
            .cloned()
            .ok_or(MapResolverError::NotFound(name.into()))
    }
}

#[derive(Debug, Error)]
pub enum MapResolverError {
    #[error("File {0} is not in the map")]
    NotFound(String),
}
