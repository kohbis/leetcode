# @param {Integer[]} nums
# @return {Integer[]}
def rearrange_array(nums)
  pos, neg = nums.partition { |n| n > 0 }
  (0...nums.size / 2).map { |i| [pos[i], neg[i]] }.flatten
end
