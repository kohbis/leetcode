# @param {Integer[][]} nums
# @return {Integer[]}
def intersection(nums)
  nums.flatten.tally.select { |_, v| v == nums.size }.keys.sort
end
