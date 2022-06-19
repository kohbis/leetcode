# @param {Integer[]} nums
# @param {Integer} key
# @return {Integer}
def most_frequent(nums, key)
  count = Hash.new(0)

  for i in 0...nums.size - 1
    if nums[i] == key
      k = nums[i + 1]
      count[k] += 1
    end
  end

  res, curr = 0, 0
  count.each do |k, v|
    res, curr = k, v if curr < v
  end

  res
end
