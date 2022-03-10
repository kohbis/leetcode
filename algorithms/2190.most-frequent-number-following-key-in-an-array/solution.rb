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

  ans, curr = 0, 0
  count.each do |k, v|
    ans, curr = k, v if curr < v
  end

  ans
end
