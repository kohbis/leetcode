# @param {Integer} num
# @return {Integer}
def count_even(num)
  res = 0

  (1..num).each do |i|
    sum = i.to_s.chars.map(&:to_i).sum
    if sum % 2 == 0
      res += 1
    end
  end

  res
end
