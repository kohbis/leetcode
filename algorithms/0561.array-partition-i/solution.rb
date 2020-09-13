# @param {Integer[]} nums
# @return {Integer}
def array_pair_sum(nums)
  res = 0
  nums.sort.each_with_index do |num, idx|
    res += num if idx % 2 == 0
  end
  res
end
