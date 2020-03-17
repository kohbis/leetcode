# @param {Integer[]} nums
# @return {Integer[]}
def decompress_rl_elist(nums)
  res = []
  nums.each_slice(2) do |cnt, num|
    cnt.times do
      res << num
    end
  end
  res
end
