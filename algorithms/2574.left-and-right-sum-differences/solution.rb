# @param {Integer[]} nums
# @return {Integer[]}
def left_rigth_difference(nums)
  (0...nums.size).map do |i|
    left = nums[...i] || []
    right = nums[i + 1..] || []
    (left.sum - right.sum).abs
  end
end
