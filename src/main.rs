use std::collections::HashMap;

mod p1;
mod p2;
mod p3;

fn main() {
    let cases: [&str; 4] = ["abcabcbb", "bbbbb", "pwwkew", "pwabwci"];
            
    for c in cases {
        p3::length_of_longest_substring(c.to_string());
    }
}

