# @param {Integer[]} nums
# @param {Integer} target
# @return {Integer[]}
def two_sum(nums, target)
  # nums.each_with_index do |n, idx|
  #   if nums.include? (target - n)
  #     return [idx, nums.index(target - n)] unless nums.index(target - n) == idx
  #   end
  # end

  h = {}
  nums.each_with_index do |n, idx|
    if h[target - n]
      return [h[target - n], idx]
    else
      h[n] = idx
    end
  end
end
