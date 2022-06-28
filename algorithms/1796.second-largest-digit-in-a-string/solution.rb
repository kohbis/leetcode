# @param {String} s
# @return {Integer}
def second_highest(s)
  digits = ('0'..'9').to_a
  nums = s.chars.select { |c| digits.include?(c) }.sort.uniq
  nums.size > 1 ? nums[-2].to_i : -1
end
