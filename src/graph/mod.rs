pub mod adjacency_list;
pub mod edge_impl;
pub mod shortest_path;

pub trait Edge {
    fn start(&self) -> usize;
    fn to(&self) -> usize;
}

pub trait WeightedEdge<T: Copy + PartialOrd + std::ops::Add<Output = T>>: Edge {
    fn weight(&self) -> T;
}

pub trait Graph<N, E: Edge> {
    fn node_count(&self) -> usize;
    fn edge_count(&self) -> usize;
    fn iter_edges(&self) -> Box<dyn Iterator<Item = &E> + '_>;
    fn get_node(&self, node_id: usize) -> Option<&N>;
    fn get_edge(&self, start: usize, to: usize) -> Option<&E>;

    /// Return an iteration of all edges incident to vertex v.
    fn edges_from(&self, node_id: usize) -> Box<dyn Iterator<Item = &E> + '_>;

    /// Add a new Node.
    fn add_node(&mut self, node_id: usize, node: N) -> bool;

    /// Add a new Edge.
    fn add_edge(&mut self, e: E);

    /// Remove node and all its incident edges from the graph.
    fn remove_node(&mut self, node_id: usize) -> bool;

    /// Remove edge from the graph.
    fn remove_edge(&mut self, start: usize, to: usize) -> bool;
}
