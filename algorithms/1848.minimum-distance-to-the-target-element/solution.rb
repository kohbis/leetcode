# @param {Integer[]} nums
# @param {Integer} target
# @param {Integer} start
# @return {Integer}
def get_min_distance(nums, target, start)
  (0...nums.size)
    .select {|i| nums[i] == target }
    .map {|n| (n - start).abs }
    .min
end
