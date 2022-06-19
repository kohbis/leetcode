# @param {String} number
# @param {Character} digit
# @return {String}
def remove_digit(number, digit)
  res = 0

  nums = number.chars
  nums.each_with_index do |n, i|
    if n == digit
      curr = (nums[0...i] + nums[i + 1..]).join.to_i
      res = [curr, res].max
    end
  end

  res.to_s
end
