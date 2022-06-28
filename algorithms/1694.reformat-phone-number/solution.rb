# @param {String} number
# @return {String}
def reformat_number(number)
  arr = []
  nums = number.gsub(/-| /, '').chars

  three_digits = nums.size / 3
  two_digits = case nums.size % 3
    when 0
      0
    when 1
      three_digits -= 1
      2
    when 2
      1
    end

  three_digits.times do
    arr << nums.shift(3).join
  end

  two_digits.times do
    arr << nums.shift(2).join
  end

  arr.join('-')
end
