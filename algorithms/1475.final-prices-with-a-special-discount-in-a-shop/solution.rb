# @param {Integer[]} prices
# @return {Integer[]}
def final_prices(prices)
  finals = []
  prices.each_with_index do |base, idx|
    discount = prices[idx+1..-1].find {|price| base >= price }.to_i
    finals << base - discount
  end
  finals
end

