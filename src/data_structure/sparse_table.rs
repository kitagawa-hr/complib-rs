pub struct SparseTable<T> {
    table: Vec<Vec<T>>,
    operator: fn(T, T) -> T,
}

impl<T: Copy> SparseTable<T> {
    pub fn new(data: &Vec<T>, operator: fn(T, T) -> T) -> Self {
        let n = data.len();
        let log_n = n.ilog2() as usize;
        let mut table = Vec::with_capacity(log_n + 1);
        table.push(data.to_owned());
        for i in 0..log_n {
            table.push(
                (0..=n - (2 << i))
                    .map(|j| operator(table[i][j], table[i][j + (1 << i)]))
                    .collect(),
            );
        }
        Self { table, operator }
    }

    pub fn fold(&self, left: usize, right: usize) -> T {
        assert!(left < right);
        let log = (right - left).ilog2() as usize;
        (self.operator)(self.table[log][left], self.table[log][right - (1 << log)])
    }
}

mod test {
    use rand::{seq::SliceRandom, Rng};

    #[test]
    fn test_sparse_table_simple() {
        let data = vec![2, 10, 1, 100];
        let sparse_table = super::SparseTable::new(&data, std::cmp::min);
        assert_eq!(sparse_table.fold(0, 1), 2);
        assert_eq!(sparse_table.fold(0, 2), 2);
        assert_eq!(sparse_table.fold(0, 3), 1);
        assert_eq!(sparse_table.fold(0, 4), 1);
        assert_eq!(sparse_table.fold(1, 2), 10);
        assert_eq!(sparse_table.fold(1, 3), 1);
        assert_eq!(sparse_table.fold(1, 4), 1);
        assert_eq!(sparse_table.fold(2, 3), 1);
        assert_eq!(sparse_table.fold(2, 4), 1);
        assert_eq!(sparse_table.fold(3, 4), 100);
    }

    #[test]
    fn test_sparse_table_random() {
        let mut rng = rand::thread_rng();
        let mut data = (0..1000).collect::<Vec<_>>();
        data.shuffle(&mut rng);
        let sparse_table = super::SparseTable::new(&data, std::cmp::min);
        for _ in 0..100 {
            let left = rng.gen_range(0..1000);
            let right = rng.gen_range(left + 1..1000);
            let actual = sparse_table.fold(left, right);
            let expected = *data[left..right].iter().min().unwrap();
            assert_eq!(actual, expected);
        }
    }
}
