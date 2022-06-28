# @param {Integer} x
# @param {Integer} y
# @return {Integer}
def hamming_distance(x, y)
  x_s = x.to_s(2).rjust(32, '0')
  y_s = y.to_s(2).rjust(32, '0')
  (0..31).select { |i| x_s[i] != y_s[i] }.count
end
