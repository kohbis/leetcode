# @param {Integer[]} nums
# @param {Integer[]} index
# @return {Integer[]}
def create_target_array(nums, index)
  (0..nums.length - 1).to_a.inject([]) { |target, i| target.insert(index[i], nums[i]) }
end
