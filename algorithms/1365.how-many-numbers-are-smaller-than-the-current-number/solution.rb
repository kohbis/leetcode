# @param {Integer[]} nums
# @return {Integer[]}
def smaller_numbers_than_current(nums)
  nums.map { |n| nums.count { |x| n > x } }
end
