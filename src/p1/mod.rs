use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m = HashMap::new();
    for (i, &n) in nums.iter().enumerate() {
        m.insert(n, i as i32);
    }
    for (i, &n) in nums.iter().enumerate() {
        let c = target - n;
        if m.contains_key(&c) && *m.get(&c).unwrap() != i as i32 {
            return Vec::from([i as i32, *m.get(&c).unwrap()]);
        }
    }
    unreachable!()
}
