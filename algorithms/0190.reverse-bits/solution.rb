# @param {Integer} n, a positive integer
# @return {Integer}
def reverse_bits(n)
  ## one-line
  n.to_s(2).reverse.ljust(32, "0").to_i(2)
end
