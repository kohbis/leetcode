# @param {Integer[]} arr
# @return {Boolean}
def three_consecutive_odds(arr)
  arr.each_cons(3) do |a, b, c|
    return true if [a, b, c].count { |i| i % 2 == 0 } == 0
  end
  false
end
