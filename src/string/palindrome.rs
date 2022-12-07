/// Calculate each palindrome radius at s by Manacher's algorithm
///
/// Palindrome radius is defined as r where s[i-r..=i+r] is a palindrome.
///
/// References:
/// * https://cp-algorithms.com/string/manacher.html
/// * https://www.slideshare.net/hcpc_hokudai/ss-121539419
///
pub fn palindrome_radiuses<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let mut left = 0;
    let mut right = 0;
    let mut rs = vec![0; s.len()];
    for i in 0..s.len() {
        if left <= i && i <= right {
            rs[i] = std::cmp::min(right - i, rs[right - (i - left)]);
        }
        rs[i] = (rs[i]..std::cmp::min(i + 1, s.len() - i))
            .take_while(|x| s[i - x] == s[i + x])
            .last()
            .unwrap();
        if i + rs[i] >= right {
            left = i - rs[i];
            right = i + rs[i];
        }
    }
    rs
}

#[cfg(test)]
mod tests {
    fn palindrome_radiuses_naive<T: PartialEq>(s: &[T]) -> Vec<usize> {
        let mut rs = vec![0; s.len()];
        for i in 0..s.len() {
            rs[i] = (0..std::cmp::min(i + 1, s.len() - i))
                .take_while(|x| s[i - x] == s[i + x])
                .last()
                .unwrap();
        }
        rs
    }
    #[test]
    fn test_longest_palindrome() {
        let cases = [
            ("abaaababa", vec![0, 1, 0, 3, 0, 1, 2, 1, 0]),
            (
                "$a$b$c$b$c$b$a$",
                vec![0, 1, 0, 1, 0, 3, 0, 7, 0, 3, 0, 1, 0, 1, 0],
            ),
            ("mississippi", vec![0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0]),
            (
                "abaaababaabaaababaabaaababaaaabbbbaaa",
                vec![
                    0, 1, 0, 3, 0, 1, 3, 1, 0, 0, 2, 0, 3, 0, 1, 3, 1, 0, 0, 2, 0, 3, 0, 1, 4, 1,
                    0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0,
                ],
            ),
        ];
        for (s, lengths) in cases {
            let chars: Vec<char> = s.chars().collect();
            let actual_naive = palindrome_radiuses_naive(&chars);
            let actual = super::palindrome_radiuses(&chars);
            assert_eq!(lengths, actual_naive);
            assert_eq!(lengths, actual);
        }
    }
}
