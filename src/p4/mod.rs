pub fn find_median_sorted_arrays(mut a: Vec<i32>, mut b: Vec<i32>) -> f64 {
    if a.len() > b.len() {
        std::mem::swap(&mut a, &mut b);
    }
    let s = (a.len() + b.len() - 1) / 2;

    let partition = binary_search(0..a.len(), |x| {
        if b[s - x] < a[x] {
            Ordering::Greater
        } else if s - x > 0 && b[s - x - 1] > a[x] {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });

    match (partition, (a.len() + b.len()) & 0b1 == 1) {
        (Ok(x), true) => a[x] as f64,
        (Err(x), true) => b[s - x] as f64,
        (Ok(x), false) => (a[x] + unwrap_min(a.get(x + 1), b.get(s - x))) as f64 / 2.0,
        (Err(x), false) => (b[s - x] + unwrap_min(a.get(x), b.get(s - x + 1))) as f64 / 2.0,
    }
}

use std::cmp::Ordering;
use std::ops::Range;
fn unwrap_min(a: Option<&i32>, b: Option<&i32>) -> i32 {
    match (a, b) {
        (Some(&a), Some(&b)) => a.min(b),
        (Some(x), None) | (None, Some(x)) => *x,
        (None, None) => panic!(),
    }
}

fn binary_search(
    Range { mut start, mut end }: Range<usize>,
    fnx: impl Fn(usize) -> Ordering,
) -> Result<usize, usize> {
    while end > start {
        let next = (start + end) / 2;
        match fnx(next) {
            Ordering::Less => start = next + 1,
            Ordering::Greater => end = next,
            Ordering::Equal => return Ok(next),
        }
    }
    return Err(end);
}
