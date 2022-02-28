# @param {Integer[]} nums
# @return {String[]}
def summary_ranges(nums)
  ans = []

  start = 0
  nums.each_index do |i|
    if nums[i] + 1 == nums[i + 1]
      next
    else
      if nums[start] == nums[i]
        ans.push nums[start].to_s
      else
        ans.push "#{nums[start]}->#{nums[i]}"
      end

      start = i + 1
    end
  end

  ans
end
