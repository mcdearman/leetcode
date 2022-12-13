use std::{cmp::max, collections::HashMap};

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut m = HashMap::new();
    let mut ans = 0;
    let mut before = -1;
    for (i, c) in s.char_indices() {
        if let Some(last) = m.insert(c, i) {
            before = max(before, last as i32);
        }
        ans = max(ans, (i as i32) - before);
    }
    ans
}

#[test]
fn test_length_of_longest_substring() {
    assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    assert_eq!(length_of_longest_substring("pwabwci".to_string()), 5);
}
