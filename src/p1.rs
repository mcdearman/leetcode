use crate::sol::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        for (i, &n) in nums.iter().enumerate() {
            map.insert(n, i as i32);
        }
        for (i, &n) in nums.iter().enumerate() {
            let c = target - n;
            if map.contains_key(&c) && *map.get(&c).unwrap() != i as i32 {
                return Vec::from([i as i32, *map.get(&c).unwrap()]);
            }
        }
        unreachable!()
    }
}
