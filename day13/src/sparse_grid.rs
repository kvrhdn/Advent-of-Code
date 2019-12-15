use common::grid::Pos;
use std::collections::HashMap;

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

    pub fn get(&self, pos: Pos) -> T {
        self.map.get(&pos).copied().unwrap_or_default()
    }

    pub fn set(&mut self, pos: Pos, value: T) {
        if value == T::default() {
            self.map.remove(&pos);
        } else {
            self.map.insert(pos, value);
        }
    }

    /// Iterates over all non-default positions.
    pub fn iterate<'a>(&'a self) -> impl Iterator<Item = &T> + 'a {
        self.map.values()
    }
}
