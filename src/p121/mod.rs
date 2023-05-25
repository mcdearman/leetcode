pub fn max_profit(prices: Vec<i32>) -> i32 {
    prices
        .iter()
        .fold((0, prices[0]), |(profit, buy), price| {
            let pr = (profit.max(*price - buy), buy.min(*price));
            println!("iter: profit = {}, buy = {}", pr.0, pr.1);
            pr
        })
        .0
}

// pub fn max_profit(prices: Vec<i32>) -> i32 {
//     let (mut profit, mut buy) = (0, prices[0]);
//     for i in 1..prices.len() {
//         profit = profit.max(prices[i] - buy);
//         buy = buy.min(prices[i]);
//     }
//     profit
// }

// [7,1,5,3,6,4]
// init: profit = 0, buy = 7
// iter 1: profit = 0, buy = 1
// iter 2: profit = 4, buy = 1

mod tests {
    #[test]
    fn example_case() {
        assert_eq!(super::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }
}
