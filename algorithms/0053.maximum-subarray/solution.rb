# @param {Integer[]} nums
# @return {Integer}
def max_sub_array(nums)
  tmp = res = nums.shift
  nums.each do |n|
    tmp = [tmp + n, n].max
    res = [tmp, res].max
  end
  res
end
