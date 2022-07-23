# @param {String} num
# @return {Boolean}
def digit_count(num)
  nums = num.chars.map(&:to_i)
  count = nums.tally

  nums.each_with_index do |n, idx|
    return false unless n == (count[idx] || 0)
  end

  true
end
