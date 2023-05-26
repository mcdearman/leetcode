pub fn max_profit(prices: Vec<i32>) -> i32 {
    prices
        .iter()
        .fold((0, prices[0]), |(profit, buy), price| {
            (profit.max(*price - buy), buy.min(*price))
        })
        .0
}

mod tests {
    #[test]
    fn example_case() {
        assert_eq!(super::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }
}
