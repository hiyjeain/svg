use std::any::Any;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::Hash;

use crate::node::{AsAny, Node};

/// A blob node.
#[derive(Clone, Debug)]
pub struct Blob {
    content: String,
}

impl Blob {
    /// Create a node.
    #[inline]
    pub fn new<T>(content: T) -> Self
    where
        T: Into<String>,
    {
        Blob {
            content: content.into(),
        }
    }
}

impl fmt::Display for Blob {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.content.fmt(formatter)
    }
}

impl AsAny for Blob {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Node for Blob {
    #[inline]
    fn get_name(&self) -> &str {
        "blob"
    }
}

impl super::NodeDefaultHash for Blob {
    #[inline]
    fn default_hash(&self, state: &mut DefaultHasher) {
        self.content.hash(state);
    }
}
