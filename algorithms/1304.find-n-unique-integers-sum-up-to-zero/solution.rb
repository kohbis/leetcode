# @param {Integer} n
# @return {Integer[]}
def sum_zero(n)
  half = n / 2
  (-half..-1).to_a + (n.odd? ? [0] : []) + (1..half).to_a
end
