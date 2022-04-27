# @param {Integer[]} nums
# @return {Integer}
def find_closest_number(nums)
  nums.sort.reverse.min_by(&:abs)
end
