# @param {Integer} n
# @return {Integer}
def alternate_digit_sum(n)
  n.to_s.chars.map(&:to_i).each_with_index.inject(0) { |res, (v, i)| res + (i % 2 == 0 ? v : -v) }
end
