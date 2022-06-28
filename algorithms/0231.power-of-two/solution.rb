# @param {Integer} n
# @return {Boolean}
def is_power_of_two(n)
  b = n.to_s(2)
  b[0] == '1' && b[1..-1].count('1') == 0
end
