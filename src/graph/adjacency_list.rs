use super::{Edge, Graph};

/// Graph implementation using adjacent list
/// Space Complexity: O(|N|+|E|)
#[derive(Debug)]
pub struct AdjListGraph<N, E> {
    pub nodes: Vec<Option<N>>,
    pub adj_list: Vec<Vec<E>>,
    node_count: usize,
    edge_count: usize,
}
impl<N, E: Edge> AdjListGraph<N, E> {
    fn node_exists(&self, node_id: usize) -> bool {
        node_id < self.nodes.len() && self.nodes[node_id].is_some()
    }
    /// create a new graph with empty nodes
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        let nodes = std::iter::repeat_with(|| None)
            .take(size)
            .collect::<Vec<_>>();
        let adj_list = std::iter::repeat_with(|| Vec::new())
            .take(size)
            .collect::<Vec<_>>();
        AdjListGraph {
            nodes,
            adj_list,
            node_count: 0,
            edge_count: 0,
        }
    }

    /// create a new graph from nodes
    #[allow(dead_code)]
    pub fn from_nodes(nodes: Vec<N>) -> Self {
        let nodes = nodes.into_iter().map(Some).collect::<Vec<_>>();
        let adj_list = std::iter::repeat_with(|| Vec::new())
            .take(nodes.len())
            .collect::<Vec<_>>();
        let node_count = nodes.len();
        AdjListGraph {
            nodes,
            adj_list,
            node_count,
            edge_count: 0,
        }
    }

    #[allow(dead_code)]
    pub fn from_edges(nodes: Vec<N>, edges: Vec<E>) -> Self {
        let mut graph = AdjListGraph::from_nodes(nodes);
        for edge in edges {
            graph.add_edge(edge);
        }
        graph
    }
}

impl<N, E: Edge> Graph<N, E> for AdjListGraph<N, E> {
    /// Complexity: O(1)
    #[inline(always)]
    fn node_count(&self) -> usize {
        self.node_count
    }

    /// Complexity: O(1)
    #[inline(always)]
    fn edge_count(&self) -> usize {
        self.edge_count
    }

    /// Complexity: O(|E|)
    fn iter_edges(&self) -> Box<dyn Iterator<Item = &E> + '_> {
        Box::new(self.adj_list.iter().flat_map(|v| v.iter()))
    }

    /// Complexity: O(1)
    fn get_node(&self, node_id: usize) -> Option<&N> {
        if self.node_exists(node_id) {
            self.nodes[node_id].as_ref()
        } else {
            None
        }
    }

    /// Complexity: O(degree)
    fn get_edge(&self, start: usize, to: usize) -> Option<&E> {
        self.adj_list[start]
            .iter()
            .find(|e| e.start() == start && e.to() == to)
    }

    /// Complexity: O(degree)
    fn edges_from(&self, node_id: usize) -> Box<dyn Iterator<Item = &E> + '_> {
        if self.node_exists(node_id) {
            Box::new(self.adj_list[node_id].iter())
        } else {
            Box::new(std::iter::empty())
        }
    }

    /// Complexity: O(1)
    #[inline(always)]
    fn add_node(&mut self, node_id: usize, node: N) -> bool {
        if self.node_exists(node_id) || node_id >= self.nodes.len() {
            false
        } else {
            self.node_count += 1;
            self.nodes[node_id] = Some(node);
            true
        }
    }

    /// Complexity: O(1)
    #[inline(always)]
    fn add_edge(&mut self, e: E) {
        self.adj_list[e.start()].push(e);
        self.edge_count += 1;
    }

    /// Complexity: O(|E|)
    fn remove_node(&mut self, node_id: usize) -> bool {
        if !self.node_exists(node_id) {
            return false;
        };
        self.nodes[node_id] = None;
        self.node_count -= 1;
        // remove all incident edges
        let diff = self
            .adj_list
            .iter_mut()
            .map(|edges| {
                let before = edges.len();
                (*edges).retain(|e| e.start() != node_id && e.to() != node_id);
                let after = edges.len();
                before - after
            })
            .sum::<usize>();
        self.edge_count -= diff;
        true
    }

    /// Complexity: O(degree)
    fn remove_edge(&mut self, start: usize, to: usize) -> bool {
        if let Some(index) = self.adj_list[start]
            .iter()
            .position(|e| e.start() == start && e.to() == to)
        {
            self.adj_list[start].remove(index);
            self.edge_count -= 1;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn setup_graph() -> AdjListGraph<usize, (usize, usize, i32)> {
        /*
        0 ---> 1 λ
        |   /  |  \
        |  /   |   \
        v v    v    \
        4 <---  3 <- 2
        */

        let graph = AdjListGraph::from_edges(
            (0..5).collect(),
            vec![
                (0, 1, 1),
                (0, 4, 4),
                (1, 3, 2),
                (1, 4, 3),
                (2, 1, 1),
                (2, 3, 1),
                (3, 4, 1),
            ],
        );
        assert_eq!(graph.node_count(), 5);
        assert_eq!(graph.edge_count(), 7);
        graph
    }
    #[test]
    fn test_iter_edges() {
        let graph = setup_graph();
        let v = graph.iter_edges().collect::<Vec<_>>();
        assert_eq!(
            v,
            vec![
                &(0, 1, 1),
                &(0, 4, 4),
                &(1, 3, 2),
                &(1, 4, 3),
                &(2, 1, 1),
                &(2, 3, 1),
                &(3, 4, 1),
            ]
        );
    }
    #[test]
    fn test_get_node() {
        let graph = setup_graph();
        assert_eq!(graph.get_node(0), Some(&0));
        assert_eq!(graph.get_node(10), None);
    }
    #[test]
    fn test_get_edge() {
        let graph = setup_graph();
        assert_eq!(graph.get_edge(0, 1), Some(&(0, 1, 1)));
        assert_eq!(graph.get_edge(1, 0), None);
        assert_eq!(graph.get_edge(0, 4), Some(&(0, 4, 4)));
    }
    #[test]
    fn test_edges_from() {
        let graph = setup_graph();
        assert_eq!(
            graph.edges_from(0).collect::<Vec<_>>(),
            vec![&(0, 1, 1), &(0, 4, 4)]
        );
        assert_eq!(
            graph.edges_from(1).collect::<Vec<_>>(),
            vec![&(1, 3, 2), &(1, 4, 3)]
        );
        assert_eq!(
            graph.edges_from(2).collect::<Vec<_>>(),
            vec![&(2, 1, 1), &(2, 3, 1)]
        );
        assert_eq!(graph.edges_from(3).collect::<Vec<_>>(), vec![&(3, 4, 1)]);
        assert_eq!(
            graph.edges_from(4).collect::<Vec<_>>(),
            Vec::<&(usize, usize, i32)>::new()
        );
    }
    #[test]
    fn test_remove_node() {
        let mut graph = setup_graph();
        assert_eq!(graph.remove_node(1), true);
        assert_eq!(graph.node_count(), 4);
        assert_eq!(graph.edge_count(), 3);
        let v = graph.iter_edges().collect::<Vec<_>>();
        assert_eq!(v, vec![&(0, 4, 4), &(2, 3, 1), &(3, 4, 1),]);
        assert_eq!(graph.remove_node(1), false);
    }
    #[test]
    fn test_remove_edge() {
        let mut graph = setup_graph();
        assert_eq!(graph.remove_edge(0, 1), true);
        assert_eq!(graph.edge_count(), 6);
        assert_eq!(graph.get_edge(0, 1), None);
    }
}
