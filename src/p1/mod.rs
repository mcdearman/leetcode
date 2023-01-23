use std::collections::{HashMap, HashSet};

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m = HashMap::new();
    for (i, &n) in nums.iter().enumerate() {
        let c = target - n;
        if m.contains_key(&c) && *m.get(&c).unwrap() != i as i32 {
            return Vec::from([i as i32, *m.get(&c).unwrap()]);
        }
        m.insert(n, i as i32);
    }
    unreachable!()
}

#[test]
fn test_two_sum() {
    assert_eq!(
        two_sum(vec![2, 7, 11, 15], 9)
            .iter()
            .copied()
            .collect::<HashSet<i32>>(),
        vec![1, 0].iter().copied().collect::<HashSet<i32>>()
    );
}
