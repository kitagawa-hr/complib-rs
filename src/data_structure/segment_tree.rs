pub struct SegmentTree<T> {
    unit: T,
    operator: fn(T, T) -> T,
    data: Vec<T>,
}

impl<T: Copy> SegmentTree<T> {
    pub fn new(v: Vec<T>, unit: T, operator: fn(T, T) -> T) -> Self {
        let mut data = vec![unit; v.len() * 2];
        for i in 0..v.len() {
            data[i + v.len()] = v[i];
        }
        for i in (1..v.len()).rev() {
            data[i] = operator(data[i << 1], data[i << 1 | 1]);
        }
        SegmentTree {
            unit,
            operator,
            data,
        }
    }

    #[inline]
    fn size(&self) -> usize {
        self.data.len() >> 1
    }

    pub fn get(&self, i: usize) -> T {
        self.data[self.size() + i]
    }

    pub fn set(&mut self, i: usize, value: T) {
        let mut index = i + self.size();
        self.data[index] = value;
        while index > 1 {
            index >>= 1;
            self.data[index] = (self.operator)(self.data[index << 1], self.data[index << 1 | 1]);
        }
    }

    pub fn fold(&self, l: usize, r: usize) -> T {
        let mut left = l + self.size();
        let mut right = r + self.size();
        let mut result_left = self.unit;
        let mut result_right = self.unit;
        while left < right {
            if left & 1 == 1 {
                result_left = (self.operator)(result_left, self.data[left]);
                left += 1;
            }
            if right & 1 == 1 {
                right -= 1;
                result_right = (self.operator)(self.data[right], result_right);
            }
            left >>= 1;
            right >>= 1;
        }
        (self.operator)(result_left, result_right)
    }
}
