# @param {Integer} num
# @return {Integer}
def maximum69_number (num)
  # num.to_s.sub('6', '9').to_i

  num_str = num.to_s
  idx = num_str.index('6')
  num_str[idx] = '9' if idx
  num_str.to_i
end
