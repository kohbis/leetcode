# @param {Integer[]} nums
# @return {Void} Do not return anything, modify nums in-place instead.
def next_permutation(nums)
  i = nums.size - 1
  while i > 0 && nums[i] <= nums[i - 1]
    i -= 1
  end

  if i == 0
    nums.reverse!
  else
    j, k = i - 1, nums.size - 1
    while nums[k] <= nums[j]
      k -= 1
    end
    nums[j], nums[k] = nums[k], nums[j]
    nums[j + 1..] = nums[j + 1..].reverse
  end

  nums
end
