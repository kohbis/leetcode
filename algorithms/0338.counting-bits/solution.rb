# @param {Integer} num
# @return {Integer[]}
def count_bits(num)
  (0..num).map { |i| i.to_s(2).count('1') }
end
