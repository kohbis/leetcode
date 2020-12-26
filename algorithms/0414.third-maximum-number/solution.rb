# @param {Integer[]} nums
# @return {Integer}
def third_max(nums)
  arr = nums.sort.uniq
  arr.size < 3 ? arr[-1] : arr[-3]
end
