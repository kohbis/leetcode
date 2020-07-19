# @param {Integer[]} nums
# @return {Integer}
def num_identical_pairs(nums)
  res = 0
  (0...nums.length).each do |i|
    (i...nums.length).each do |j|
      res += 1 if nums[i] == nums[j] && i < j
    end
  end
  res
end
