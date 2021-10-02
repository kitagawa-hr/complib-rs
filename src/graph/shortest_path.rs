use super::graph::{Graph, WeightedEdge};
use std::cmp::Reverse;

// Result Container for Single Source Shortest Path
pub struct SSSPResult<W> {
    start: usize,
    pub costs: Vec<W>,
    pre_nodes: Vec<usize>,
}
impl<W> SSSPResult<W> {
    // build shortest path to goal
    // Complexity: O(|path|)
    pub fn build_path(&self, goal: usize) -> Vec<usize> {
        let mut cur = goal;
        let mut path = vec![goal];
        while cur != self.start {
            path.push(self.pre_nodes[cur]);
            cur = self.pre_nodes[cur];
        }
        path.reverse();
        path
    }
}

// Dijkstra's shortest path algorithm.
// Complexity: O(|E|log|N|)
#[allow(dead_code)]
pub fn dijkstra<
    W: Copy + Ord + PartialOrd + std::ops::Add<Output = W>,
    N,
    E: WeightedEdge<W>,
    G: Graph<N, E>,
>(
    graph: &G,
    start: usize,
    goal: Option<usize>,
    zero: W,
    inf: W,
) -> SSSPResult<W> {
    // (cost, position, pre_node)
    let mut costs: Vec<_> = (0..graph.node_count()).map(|_| inf).collect();
    let mut pre_nodes: Vec<_> = (0..graph.node_count()).map(|i| i).collect();
    let mut visit_next = std::collections::BinaryHeap::new();

    costs[start] = zero;
    visit_next.push((Reverse(zero), start));

    while let Some((Reverse(cur_cost), cur_node)) = visit_next.pop() {
        if cur_cost > costs[cur_node] {
            continue;
        }
        if goal.is_some() && cur_node == goal.unwrap() {
            break;
        }
        graph.edges_from(cur_node).for_each(|edge| {
            let next_cost = cur_cost + edge.weight();
            let next_node = edge.to();
            if next_cost < costs[next_node] {
                visit_next.push((Reverse(next_cost), next_node));
                costs[next_node] = next_cost;
                pre_nodes[next_node] = cur_node;
            }
        })
    }
    SSSPResult {
        start,
        costs,
        pre_nodes,
    }
}

#[cfg(test)]
mod tests {
    use super::super::adjacency_list::AdjListGraph;
    use super::*;
    fn setup_graph() -> AdjListGraph<usize, (usize, usize, usize)> {
        // this example is from https://doc.rust-lang.org/std/collections/binary_heap/index.html
        //
        //                  7
        //          +-----------------+
        //          |                 |
        //          v   1        2    |  2
        //          0 -----> 1 -----> 3 ---> 4
        //          |        ^        ^      ^
        //          |        | 1      |      |
        //          |        |        | 3    | 1
        //          +------> 2 -------+      |
        //           10      |               |
        //                   +---------------+
        //
        let mut graph = AdjListGraph::new(10);
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
        for i in 0..5 {
            assert_eq!(graph.add_node(i, i), true);
        }
        assert_eq!(graph.node_count(), 5);
        graph.add_edge((0, 1, 1));
        graph.add_edge((0, 2, 10));
        graph.add_edge((1, 3, 2));
        graph.add_edge((2, 1, 1));
        graph.add_edge((2, 3, 3));
        graph.add_edge((2, 4, 1));
        graph.add_edge((3, 0, 7));
        graph.add_edge((3, 4, 2));
        assert_eq!(graph.edge_count(), 8);
        graph
    }
    #[test]
    fn test_dijkstra() {
        let graph = setup_graph();
        const INF: usize = std::usize::MAX;
        let result = dijkstra(&graph, 0, None, 0, INF);
        assert_eq!(result.costs, vec![0, 1, 10, 3, 5]);
        assert_eq!(result.build_path(0), vec![0]);
        assert_eq!(result.build_path(1), vec![0, 1]);
        assert_eq!(result.build_path(2), vec![0, 2]);
        assert_eq!(result.build_path(3), vec![0, 1, 3]);
        assert_eq!(result.build_path(4), vec![0, 1, 3, 4]);
        let result = dijkstra(&graph, 2, None, 0, INF);
        assert_eq!(result.costs, vec![10, 1, 0, 3, 1]);
        assert_eq!(result.build_path(1), vec![2, 1]);
        assert_eq!(result.build_path(2), vec![2]);
        assert_eq!(result.build_path(4), vec![2, 4]);
    }
}
