use std::ops::{Add, Div, Sub};

/// Examines the partitioned range [first, last) and locates the first index of the second partition.
/// The range [first, last) is assumed too be partitioned by given predicate.
///
/// # Arguments
/// * `first` - first index of the partitioned range to examine
/// * `last` - last index of the partitioned range to examine
/// * `pred` - unary predicate which returns true for the elements found in the beginning of the range
///
pub fn partition_point<T, F>(first: &T, last: &T, pred: F) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + From<u8> + PartialEq + Copy,
    F: Fn(&T) -> bool,
{
    let mut left = *first;
    let mut right = *last;
    while left != right {
        let mid = left + (right - left) / T::from(2_u8);
        if pred(&mid) {
            left = mid + T::from(1_u8);
        } else {
            right = mid;
        }
    }
    left
}

pub trait BinarySearch<T: std::cmp::Ord> {
    fn lower_bound(&self, value: &T) -> usize;
    fn upper_bound(&self, value: &T) -> usize;
    fn equal_range(&self, value: &T) -> (usize, usize) {
        (self.lower_bound(value), self.upper_bound(value))
    }
}

impl<T: std::cmp::Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, value: &T) -> usize {
        partition_point(&0, &self.len(), |&x| self[x] < *value)
    }
    fn upper_bound(&self, value: &T) -> usize {
        partition_point(&0, &self.len(), |&x| self[x] <= *value)
    }
}

#[test]
fn test_binary_search() {
    let vec = vec![1, 2, 4, 6, 7, 12, 54, 60];

    assert_eq!(partition_point(&0, &3, |&i| vec[i] < 4), 2);
    assert_eq!(partition_point(&0, &3, |&i| vec[i] <= 4), 3);
    assert_eq!(partition_point(&0, &3, |&i| vec[i] < 100), 3);
    assert_eq!(partition_point(&0, &7, |&i| vec[i] < 100), 7);
    assert_eq!(vec.lower_bound(&1), 0);
    assert_eq!(vec.lower_bound(&4), 2);
    assert_eq!(vec.lower_bound(&100), 8);
    assert_eq!(vec.upper_bound(&1), 1);
    assert_eq!(vec.upper_bound(&4), 3);
    assert_eq!(vec.upper_bound(&100), 8);
    assert_eq!(vec.equal_range(&6), (3, 4));
    assert_eq!(vec.equal_range(&5), (3, 3));
    assert_eq!(vec.equal_range(&100), (8, 8));
}
