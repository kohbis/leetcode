# @param {Integer} low
# @param {Integer} high
# @return {Integer}
def count_odds(low, high)
  diff = high - low
  sum = diff / 2

  if diff % 2 == 1
    sum += 1
  elsif low % 2 == 1
    sum += 1
  end

  sum
end
