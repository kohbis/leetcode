# @param {Integer[]} nums
# @return {Integer}
def array_sign(nums)
  negative_count = 0
  nums.each do |n|
    return 0 if n == 0
    negative_count += 1 if 0 > n
  end
  negative_count % 2 == 0 ? 1 : -1
end
