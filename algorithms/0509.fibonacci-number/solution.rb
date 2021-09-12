# @param {Integer} n
# @return {Integer}
def fib(n)
  a, b = 0, 1
  n.times { |i| a, b = b, a + b }
  a
end
