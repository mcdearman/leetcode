# 1. Two Sum
## Algorithm
First we create a hashmap `m` with the values of `nums` as keys and their indices as values by iterating through `nums`. Since we can assume there exists exactly one solution, we know that if we visit each element `n` of `nums`, calculating the difference between the target and `n`, we are guaranteed to find the value that, when added to `n`, yields the target by the time we finish iterating. Thus, we iterate over `nums` again, this time calculating the value `target - n` and assigning it to a variable `c`. Then, if `c` is a key in `m` and is not at the current index (this ensures we can't use the same element twice), we return a vector containing our current index and the index of `c`. 

### Initial Values
- `m` - A hash table where the keys are the values of `nums` and the values are their indices

## Rust Implementation
```rust
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
    unreachable!() /* <-- Rust needs this because it can't know this is unreachable at 
    compile time. We know the input is guaranteed to have a solution so we can assume 
    this is safe. */
}
