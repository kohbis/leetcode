# @param {Integer[]} nums
# @return {Integer}
def missing_number(nums)
  # (0..nums.size).each {|i| return i unless nums.include?(i)}
  
  (0..nums.size).sum - nums.sum
end
