# @param {Integer} num
# @return {Integer}
def count_even(num)
  ans = 0

  (1..num).each do |i|
    sum = i.to_s.chars.map(&:to_i).sum
    if sum % 2 == 0
      ans += 1
    end
  end

  ans
end
