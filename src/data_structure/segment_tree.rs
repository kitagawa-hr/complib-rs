pub struct SegmentTree<T> {
    unit: T,
    combine: fn(T, T) -> T,
    data: Vec<T>,
    size: usize,
}

impl<T: Copy> SegmentTree<T> {
    pub fn new(vec: &mut Vec<T>, unit: T, combine: fn(T, T) -> T) -> Self {
        let size = vec.len();
        let mut data = vec![unit; size];
        data.append(vec);
        for i in (0..size).rev() {
            data[i] = combine(data[i << 1], data[i << 1 | 1]);
        }
        SegmentTree {
            unit,
            combine,
            data,
            size,
        }
    }

    pub fn get(&self, i: usize) -> T {
        self.data[self.size + i]
    }

    pub fn update(&mut self, i: usize, value: T) {
        let mut index = i + self.size;
        self.data[index] = value;
        while index > 1 {
            index >>= 1;
            self.data[index] = (self.combine)(self.data[index << 1], self.data[index << 1 | 1]);
        }
    }
    pub fn query(&self, l: usize, r: usize) -> T {
        let mut left = l + self.size;
        let mut right = r + self.size;
        let mut result = self.unit;
        while left < right {
            if left & 1 == 1 {
                result = (self.combine)(self.data[left], result);
                left += 1;
            }
            if right & 1 == 1 {
                right -= 1;
                result = (self.combine)(result, self.data[right]);
            }
            left >>= 1;
            right >>= 1;
        }
        result
    }
}
