# @param {Integer[]} nums
# @return {Integer}
def find_the_array_conc_val(nums)
  res = 0
  while nums.size > 0
    res += nums.size > 1 ? "#{nums.shift}#{nums.pop}".to_i : nums.pop
  end
  res
end
