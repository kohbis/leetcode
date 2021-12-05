# @param {Integer[]} nums
# @param {Integer} target
# @return {Integer[]}
def target_indices(nums, target)
  target_count, smaller_count = 0, 0

  nums.each do |n|
    target_count += 1 if target == n
    smaller_count += 1 if target > n
  end

  (smaller_count...smaller_count + target_count).to_a
end
