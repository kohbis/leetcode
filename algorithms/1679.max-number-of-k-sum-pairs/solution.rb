# @param {Integer[]} nums
# @param {Integer} k
# @return {Integer}
def max_operations(nums, k)
  h = Hash.new(0)
  count = 0

  nums.each do |num|
    if h[k - num] > 0
      count += 1
      h[k - num] -= 1
    else
      h[num] += 1
    end
  end

  count
end
