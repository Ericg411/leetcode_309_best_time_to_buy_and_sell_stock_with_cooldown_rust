pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut cool_down: i32 = 0;
        let mut sell: i32 = 0;
        let mut hold: f64 = f64::NEG_INFINITY;

        for price in prices {
            let prev_cool_down = cool_down;
            let prev_sell = sell;
            let prev_hold = hold;

            cool_down = std::cmp::max(prev_cool_down, prev_sell);
            sell = prev_hold as i32 + price;
            hold = std::cmp::max(prev_hold as i32, prev_cool_down - price) as f64;
        }

        std::cmp::max(sell, cool_down)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        let test_cases = vec![
            (vec![1, 2, 3, 0, 2], 3),
            (vec![1], 0),             
            (vec![2, 1], 0),           
            (vec![1, 2, 4, 2, 5, 3], 4), 
            (vec![3, 2, 6, 5, 0, 3], 7), 
        ];

        for (prices, expected) in test_cases {
            let result = Solution::max_profit(prices);
            assert_eq!(result, expected);
        }
    }
}
