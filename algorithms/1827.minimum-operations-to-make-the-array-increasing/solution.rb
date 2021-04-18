# @param {Integer[]} nums
# @return {Integer}
def min_operations(nums)
  count = prev = 0
  nums.each do |num|
    unless num > prev
      count += (prev + 1) - num
      prev += 1
    else
      prev = num
    end
  end
  count
end
