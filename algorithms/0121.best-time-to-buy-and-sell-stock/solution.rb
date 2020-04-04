# @param {Integer[]} prices
# @return {Integer}
def max_profit(prices)
  max_profit = 0
  len = prices.length
  for i in 0..(len-2)
    max = prices[i+1...len].max
    diff = max - prices[i]
    max_profit = diff if diff > max_profit && diff.positive?
  end
  max_profit
end
