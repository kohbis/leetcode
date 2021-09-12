# @param {Integer[]} nums
# @return {Integer[][]}
def subsets(nums)
  res = [[]]
  for i in 1..nums.length
    res.concat(nums.combination(i).to_a)
  end
  res
end
