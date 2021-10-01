use super::graph::{Graph, WeightedEdge};
use std::cmp::Reverse;

fn build_path(pre_nodes: &Vec<usize>, start: usize, goal: usize) -> Vec<usize> {
    let mut cur = goal;
    let mut path = vec![goal];
    while cur != start {
        path.push(pre_nodes[cur]);
        cur = pre_nodes[cur];
    }
    path.reverse();
    path
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
    goal: usize,
    zero: W,
    inf: W,
) -> Option<(W, Vec<usize>)> {
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
        if cur_node == goal {
            let path = build_path(&pre_nodes, start, goal);
            return Some((cur_cost, path));
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
    None
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
        assert_eq!(dijkstra(&graph, 0, 1, 0, usize::MAX), Some((1, vec![0, 1])));
        assert_eq!(
            dijkstra(&graph, 0, 3, 0, usize::MAX),
            Some((3, vec![0, 1, 3]))
        );
        assert_eq!(dijkstra(&graph, 3, 0, 0, usize::MAX), Some((7, vec![3, 0])));
        assert_eq!(
            dijkstra(&graph, 0, 4, 0, usize::MAX),
            Some((5, vec![0, 1, 3, 4]))
        );
        assert_eq!(dijkstra(&graph, 4, 0, 0, usize::MAX), None);
    }
}
