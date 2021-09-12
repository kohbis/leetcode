# @param {Integer[]} nums
# @return {Integer}
def majority_element(nums)
  nums.group_by(&:itself).sort_by { |_, v| -v.count }[0][0]
end
