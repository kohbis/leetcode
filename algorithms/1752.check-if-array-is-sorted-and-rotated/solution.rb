# @param {Integer[]} nums
# @return {Boolean}
def check(nums)
  rotate = false
  max = 0

  (0..nums.size - 1).each do |i|
    if nums[i] >= nums[i - 1]
      return false if rotate && nums[i] > max
    else
      return false if rotate

      rotate = true
      max = nums[i - 1]
    end
  end

  true
end
