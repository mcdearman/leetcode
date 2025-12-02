pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
    let mut cur = 0;
    let mut res = vec![];
    for n in nums {
        cur = (cur << 1 | n) % 5;
        res.push(cur % 5 == 0);
    }
    res
}

//  10101111
//  00001111
