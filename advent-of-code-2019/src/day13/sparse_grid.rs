use crate::grid::Pos;
use std::collections::HashMap;

/// A sparse grid stores a grid in a map, it can hold an infinite grid and it
/// optimized for random access. Default values are not stored in the map,
/// resulting in a smaller memory footprint.
pub struct SparseGrid<T> {
    map: HashMap<Pos, T>,
}

impl<T> SparseGrid<T>
where
    T: Copy + Default + PartialEq,
{
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Get the value at the given position.
    pub fn get(&self, pos: Pos) -> T {
        self.map.get(&pos).copied().unwrap_or_default()
    }

    /// Store the given value at the given position.
    pub fn set(&mut self, pos: Pos, value: T) {
        if value == T::default() {
            self.map.remove(&pos);
        } else {
            self.map.insert(pos, value);
        }
    }

    /// Find any position that contains the given value. Returns the first it
    /// finds. Positions with the default value are not stored explicitly and
    /// can not be retrieved.
    pub fn find(&self, value_to_find: T) -> Option<Pos> {
        self.map
            .iter()
            .filter(|&(_, value)| *value == value_to_find)
            .map(|(&pos, _)| pos)
            .next()
    }

    /// Iterates over all non-default positions. Positions with the default
    /// value are not stored explicitly and can not be retrieved.
    pub fn iterate<'a>(&'a self) -> impl Iterator<Item = &T> + 'a {
        self.map.values()
    }
}
