# @param {String} number
# @param {Character} digit
# @return {String}
def remove_digit(number, digit)
  ans = 0

  nums = number.chars
  nums.each_with_index do |n, i|
    if n == digit
      curr = (nums[0...i] + nums[i + 1..]).join.to_i
      ans = [curr, ans].max
    end
  end

  ans.to_s
end
