# @param {Integer} n
# @return {Integer}
def sum_of_multiples(n)
  x = 3 * 5 * 7
  (1..n).sum { |i| i.gcd(x) > 1 ? i : 0 }
end
