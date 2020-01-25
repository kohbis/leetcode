# @param {Integer[]} nums
# @param {Integer} target
# @return {Integer[]}
def two_sum(nums, target)
  nums.each_with_index do |n, idx|
    if nums.include? (target - n)
      return [idx, nums.index(target - n)] unless nums.index(target - n) == idx
    end
    next
  end
end
