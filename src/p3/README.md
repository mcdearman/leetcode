# 3. Longest Substring Without Repeating Characters
## Algorithm
We iterate through the enumerated string and check if the current character is already in our hash table. If it is, we move the start of our window to the max of the current start and the index of the character in our hash table. This slides the start of the window to the character immediately after the last instance of our duplicate. We also update the character's value in the hash table to have the index of the current character. Finally, we set our answer to be the max of the current answer and the range of our current window (the difference the current index and the start of our window).

#### Initial Values
- `m` - A hash table where the keys are our characters and the values are the indices where each character was last seen
- `ans` - A 32-bit integer representing our final answer
- `before` - A 32-bit integer representing the start of our sliding window

## Rust Implementation

```rust
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
```
