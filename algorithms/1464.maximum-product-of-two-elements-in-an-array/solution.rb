# @param {Integer[]} nums
# @return {Integer}
def max_product(nums)
  nums.max(2).inject(1) { |res, num| res * (num - 1) }
end
