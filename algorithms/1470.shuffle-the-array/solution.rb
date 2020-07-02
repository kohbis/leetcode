# @param {Integer[]} nums
# @param {Integer} n
# @return {Integer[]}
def shuffle(nums, n)
  n.times.flat_map {|i| [nums[i], nums[i+n]] }
end
