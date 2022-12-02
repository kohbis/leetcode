# @param {Integer[]} nums
# @return {Integer}
def average_value(nums)
  arr = nums.select { |num| num % 6 == 0 }
  arr.empty? ? 0 : arr.sum / arr.size
end
