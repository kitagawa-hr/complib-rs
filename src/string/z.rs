/// Implementation of Z algorithm
///
/// References:
///   * <https://codeforces.com/blog/entry/3107>
///   * <https://snuke.hatenablog.com/entry/2014/12/03/214243>
///
pub fn z_algorithm<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let mut left = 0;
    let mut right = 0;
    let mut lengths = vec![0; s.len()];
    for i in 1..s.len() {
        if left <= i && i <= right {
            lengths[i] = std::cmp::min(right - i + 1, lengths[i - left]);
        }
        lengths[i] = (lengths[i]..s.len() - i)
            .take_while(|&x| s[x] == s[i + x])
            .last()
            .map_or(lengths[i], |x| x + 1);
        if right <= i + lengths[i] {
            left = i;
            right = i + lengths[i] - 1;
        }
    }
    lengths[0] = s.len();
    lengths
}

#[cfg(test)]
mod tests {
    fn z_algorithm_naive<T: PartialEq>(s: &[T]) -> Vec<usize> {
        let mut lengths = vec![0; s.len()];
        for i in 0..s.len() {
            lengths[i] = (0..s.len() - i).take_while(|&x| s[x] == s[i + x]).count()
        }
        lengths
    }
    #[test]
    fn test_longest_palindrome() {
        let cases = [(
            "aabbccaabbccdd",
            vec![14, 1, 0, 0, 0, 0, 6, 1, 0, 0, 0, 0, 0, 0],
        ),
        ("ababacaca", vec![9,0,3,0,1,0,1,0,1]),
        ("aaaaaaaaa", vec![9,8,7,6,5,4,3,2,1]),
        ("ccaaacabccaabcbcacbbbbbacaccccbbcacbacccccbbbacaabaabacacbbccccbaaacabbbbacbabbcbcbbcaabbacccacabcac",
        vec![100,1,0,0,0,1,0,0,4,1,0,0,0,1,0,1,0,1,0,0,0,0,0,0,1,0,2,2,2,1,0,0,1,0,1,0,0,2,2,2,2,1,0,0,0,0,1,0,0,0,0,0,0,0,1,0,1,0,0,2,2,2,1,0,0,0,0,1,0,0,0,0,0,0,1,0,0,0,0,1,0,1,0,0,1,0,0,0,0,0,2,3,1,0,1,0,0,1,0,1,])
        ];
        for (s, lcps) in cases {
            let chars: Vec<char> = s.chars().collect();
            let actual_naive = z_algorithm_naive(&chars);
            let actual = super::z_algorithm(&chars);
            assert_eq!(lcps, actual_naive);
            assert_eq!(lcps, actual);
        }
    }
}
