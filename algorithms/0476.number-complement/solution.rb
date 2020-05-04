# @param {Integer} num
# @return {Integer}
def find_complement(num)
  num.to_s(2).chars.map {|c| c == '0' ? '1' : '0' }.join.to_i(2)
end

