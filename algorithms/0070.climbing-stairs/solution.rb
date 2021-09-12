# @param {Integer} n
# @return {Integer}
def climb_stairs(n)
  # fibonacci sequence

  # a = b = 1
  # n.times {|i| a, b = b, a+b }
  # a

  a = [1, 1]
  (2..n).each { |i| a[i] = a[i - 2] + a[i - 1] }
  a[-1]
end
