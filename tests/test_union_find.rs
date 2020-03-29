use itertools::Itertools;
use complib_rs::data_structure::union_find::UnionFind;

use proconio::input;
use proconio::source::auto::AutoSource;

mod util;

#[test]
fn union_find_test() {
    // AOJ DSL_1_A
    // http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_A
    util::test_solution(
        "tests/assets/union_find",
        |input_str| {
            let mut source = AutoSource::from(input_str);
            input! {
                from &mut source,
                n: usize,
                q: usize,
            }
            let mut uf = UnionFind::new(n);
            let mut v = Vec::new();
            for _ in 0..q {
                input! {
                    from &mut source,
                    com: usize,
                    x: usize,
                    y: usize,
                }
                if com == 0 {
                    uf.unite(x, y, 0);
                } else {
                    v.push(if uf.equiv(x, y) { 1 } else { 0 })
                }
            }
            v.iter().join("\n")
        },
    );
}
