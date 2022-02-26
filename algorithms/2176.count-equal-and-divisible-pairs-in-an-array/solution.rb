# @param {Integer[]} nums
# @param {Integer} k
# @return {Integer}
def count_pairs(nums, k)
  count = 0

  for i in 0...nums.size - 1
    for j in i + 1...nums.size
      if nums[i] == nums[j] && i * j % k == 0
        count += 1
      end
    end
  end

  count
end
