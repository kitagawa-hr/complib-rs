use complib_rs::prelude::*;
use itertools::Itertools;

use proconio::input;
use proconio::source::auto::AutoSource;

mod util;

#[test]
fn dijkstra_test() {
    // AOJ GRL_1_A
    // https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_A
    util::test_solution("tests/assets/shortest_path", |input_str| {
        let mut source = AutoSource::from(input_str);
        input! {
            from &mut source,
            v: usize,
            e: usize,
            start: usize,
        }
        let mut g = AdjListGraph::new(v);
        for i in 0..v {
            g.add_node(i, i);
        }
        for _ in 0..e {
            input! {
                from &mut source,
                s: usize,
                t: usize,
                w: usize
            }
            g.add_edge((s, t, w));
        }
        const INF: usize = usize::MAX;
        let result = shortest_path::dijkstra(&g, start, None, 0, INF);
        result
            .costs
            .iter()
            .map(|&c| {
                if c == INF {
                    "INF".to_string()
                } else {
                    c.to_string()
                }
            })
            .join("\n")
    });
}
