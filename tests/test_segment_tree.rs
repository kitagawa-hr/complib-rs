use complib_rs::data_structure::segment_tree::SegmentTree;
use itertools::Itertools;

use proconio::input;
use proconio::source::auto::AutoSource;

mod util;

#[test]
fn segment_tree_test() {
    // AOJ DSL_2_A
    // https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A&lang=jp
    util::test_solution("tests/assets/range_min_query", |input_str| {
        let mut source = AutoSource::from(input_str);
        input! {
            from &mut source,
            n: usize,
            q: usize,
        }
        let mut v = vec![(1 << 31) - 1; n];
        let mut st = SegmentTree::new(&mut v, (1 << 31) - 1, std::cmp::min);
        let mut v = Vec::new();
        for _ in 0..q {
            input! {
                from &mut source,
                com: usize,
                x: usize,
                y: usize,
            }
            if com == 0 {
                st.update(x, y);
            } else {
                v.push(st.query(x, y + 1));
            }
        }
        v.iter().join("\n")
    });
}
