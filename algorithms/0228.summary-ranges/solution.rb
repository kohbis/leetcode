# @param {Integer[]} nums
# @return {String[]}
def summary_ranges(nums)
  res = []

  start = 0
  nums.each_index do |i|
    if nums[i] + 1 == nums[i + 1]
      next
    else
      if nums[start] == nums[i]
        res.push nums[start].to_s
      else
        res.push "#{nums[start]}->#{nums[i]}"
      end

      start = i + 1
    end
  end

  res
end
