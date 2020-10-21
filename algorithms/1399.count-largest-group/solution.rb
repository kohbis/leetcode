# @param {Integer} n
# @return {Integer}
def count_largest_group(n)
  h = {}
  (1..n).each do |num|
    k = num.digits.sum
    h[k] ||= 0
    h[k] += 1
  end
  values = h.values

  values.count(values.max)
end
