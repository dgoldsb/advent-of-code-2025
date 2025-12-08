use std::collections::HashMap;
use std::hash::Hash;

pub struct UnionFind<T> {
    parent: Vec<usize>,
    pub size: Vec<usize>,
    indices: HashMap<T, usize>,
    elements: Vec<T>,
}

impl<T: Eq + Hash + Clone> UnionFind<T> {
    pub fn new() -> Self {
        Self {
            parent: Vec::new(),
            size: Vec::new(),
            indices: HashMap::new(),
            elements: Vec::new(),
        }
    }

    // Ensure an element is in the structure and get its index
    fn index(&mut self, x: T) -> usize {
        if let Some(&i) = self.indices.get(&x) {
            return i;
        }

        let i = self.parent.len();
        self.parent.push(i);
        self.size.push(1);
        self.indices.insert(x.clone(), i);
        self.elements.push(x);
        i
    }

    // Find with path compression
    pub fn find(&mut self, x: T) -> T {
        let idx = self.index(x.clone());
        let root_idx = self.find_index(idx);
        self.elements[root_idx].clone()
    }

    fn find_index(&mut self, idx: usize) -> usize {
        if self.parent[idx] != idx {
            self.parent[idx] = self.find_index(self.parent[idx]);
        }
        self.parent[idx]
    }

    // Union by size
    pub fn union(&mut self, x: T, y: T) {
        let idx_x = self.index(x);
        let idx_y = self.index(y);
        self.union_indices(idx_x, idx_y);
    }

    fn union_indices(&mut self, mut idx_x: usize, mut idx_y: usize) {
        idx_x = self.find_index(idx_x);
        idx_y = self.find_index(idx_y);
        if idx_x == idx_y {
            return;
        }

        if self.size[idx_x] < self.size[idx_y] {
            std::mem::swap(&mut idx_x, &mut idx_y);
        }

        self.parent[idx_y] = idx_x;
        self.size[idx_x] += self.size[idx_y];
    }
}