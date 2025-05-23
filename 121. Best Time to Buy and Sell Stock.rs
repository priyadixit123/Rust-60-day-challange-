https://leetcode.com/problems/best-time-to-buy-and-sell-stock/?envType=study-plan-v2&envId=top-interview-150


Sol

         impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX;
        let mut max_profit = 0;

        for price in prices {
            min_price = min_price.min(price);
            max_profit = max_profit.max(price - min_price);
        }

        max_profit
    }
}
