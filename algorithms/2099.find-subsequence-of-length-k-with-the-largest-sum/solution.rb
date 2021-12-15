# @param {Integer[]} nums
# @param {Integer} k
# @return {Integer[]}
def max_subsequence(nums, k)
  res = []

  target = nums.sort[-k..]
  for i in 0...nums.size
    idx = target.index(nums[i])
    if idx
      res << nums[i]
      target.delete_at(idx)

      break if target.size == 0
    end
  end

  res
end
