# @param {Integer[]} prices
# @return {Integer}
def max_profit(prices)
  profit = 0
  prices.each_cons(2) do |sell, buy|
    profit += buy - sell if (buy - sell).positive?
  end
  profit
end
