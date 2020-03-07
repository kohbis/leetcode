# @param {Integer[]} nums
# @param {Integer} target
# @return {Integer}
def search_insert(nums, target)
  return nums.size if target > nums[-1]
  nums.find_index { |n| n >= target }
end
