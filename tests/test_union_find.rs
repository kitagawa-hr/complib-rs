use complib_rs::data_structure::union_find::UnionFind;
use itertools::Itertools;

use proconio::input;
use proconio::source::auto::AutoSource;

mod util;

#[test]
fn union_find_test() {
    // AOJ DSL_1_A
    // http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_A
    util::test_solution("tests/assets/union_find", |input_str| {
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
                v.push(i32::from(uf.equiv(x, y)));
            }
        }
        v.iter().join("\n")
    });
}

#[test]
fn union_find_weight_test() {
    // AOJ 1330
    // http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1330
    util::test_solution("tests/assets/union_find_weight", |input_str| {
        let mut source = AutoSource::from(input_str);
        let mut v = Vec::new();
        loop {
            input! {
                from &mut source,
                n: usize,
                m: usize,
            }
            if (n == 0) && (m == 0) {
                break;
            }
            let mut uf = UnionFind::new(n);
            for _ in 0..m {
                input! {
                    from &mut source,
                    query: char
                }
                if query == '!' {
                    input! {
                        from &mut source,
                        a: usize,
                        b: usize,
                        w: i64,
                    }
                    uf.unite(a - 1, b - 1, w);
                } else {
                    input! {
                        from &mut source,
                        a: usize,
                        b: usize,
                    }
                    if uf.equiv(a - 1, b - 1) {
                        v.push(format!("{}", uf.diff(a - 1, b - 1)));
                    } else {
                        v.push(String::from("UNKNOWN"));
                    }
                }
            }
        }
        v.iter().join("\n")
    })
}
