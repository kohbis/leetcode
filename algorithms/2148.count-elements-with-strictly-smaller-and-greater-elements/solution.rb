# @param {Integer[]} nums
# @return {Integer}
def count_elements(nums)
  count = 0
  nums_min, nums_max = nums.min, nums.max

  nums.each do |n|
    if nums_min < n && n < nums_max
      count += 1
    end
  end

  count
end
