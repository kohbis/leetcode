# @param {Integer} num
# @return {Integer}
def minimum_sum(num)
  nums = num.to_s.chars.sort.map(&:to_i)
  nums[0] * 10 + nums[1] * 10 + nums[2] + nums[3]
end
