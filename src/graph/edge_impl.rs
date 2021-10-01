pub use super::graph::{Edge, WeightedEdge};

impl Edge for (usize, usize) {
    fn start(&self) -> usize {
        self.0
    }

    fn to(&self) -> usize {
        self.1
    }
}

impl WeightedEdge<usize> for (usize, usize) {
    fn weight(&self) -> usize {
        1
    }
}

impl<T> Edge for (usize, usize, T) {
    fn start(&self) -> usize {
        self.0
    }

    fn to(&self) -> usize {
        self.1
    }
}

impl<T: Copy + Ord + std::ops::Add<Output = T>> WeightedEdge<T> for (usize, usize, T) {
    fn weight(&self) -> T {
        self.2
    }
}
