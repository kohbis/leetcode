impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0i32;
        let mut buy_price = i32::MAX;

        for price in prices {
            buy_price = buy_price.min(price);
            max_profit = max_profit.max(price - buy_price);
        }

        max_profit
    }
}
