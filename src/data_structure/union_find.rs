use std::ops::{Add, Sub, Neg};

pub struct UnionFind<T> {
    parents: Vec<usize>,
    pub ranks: Vec<usize>,
    pub weights: Vec<T>,
}

impl<T> UnionFind<T>
    where T: Add<Output=T> + Clone + Copy + From<u8> + Sub<Output=T>
{
    pub fn new(n: usize) -> Self {
        UnionFind {
            parents: (0..n).map(|i| i).collect(),
            ranks: vec![1; n],
            weights: vec![T::from(0u8); n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x == self.parents[x] { return x; }
        let root = self.find(self.parents[x]);
        self.weights[x] = self.weights[x] + self.weights[self.parents[x]];
        self.parents[x] = root;
        return root;
    }

    pub fn unite(&mut self, x: usize, y: usize, weight: T) -> bool {
        // weights[x]+ weight = weights[y]
        let x_root = self.find(x);
        let y_root = self.find(y);
        if x_root == y_root { return false; }
        let (smaller, bigger, new_weight) = if self.size(x_root) >= self.size(y_root) {
            (y_root, x_root, self.weights[x] - self.weights[y] + weight)
        } else {
            (x_root, y_root, self.weights[y] - self.weights[x] - weight)
        };
        self.parents[smaller] = bigger;
        self.ranks[bigger] += self.ranks[smaller];
        self.weights[smaller] = new_weight;
        true
    }

    pub fn diff(&mut self, x: usize, y: usize) -> T {
        self.find(x);
        self.weights[y] - self.weights[x]
    }

    pub fn size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.ranks[root]
    }

    pub fn equiv(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}
