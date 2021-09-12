# @param {Integer} n
# @return {Integer}
def bitwise_complement(n)
  # n.to_s(2).chars.map {|c| c == '0' ? '1' : '0' }.join.to_i(2)

  n.to_s(2).tr("01", "10").to_i(2)
end
