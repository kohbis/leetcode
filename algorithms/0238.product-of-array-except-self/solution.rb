# @param {Integer[]} nums
# @return {Integer[]}
def product_except_self(nums)
  res = []

  left = 1
  0.upto(nums.size - 1).each do |i|
    res[i] = left
    left *= nums[i]
  end

  right = 1
  (nums.size - 1).downto(0).each do |i|
    res[i] *= right
    right *= nums[i]
  end

  res
end
