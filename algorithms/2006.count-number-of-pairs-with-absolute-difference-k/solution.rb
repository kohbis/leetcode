# @param {Integer[]} nums
# @param {Integer} k
# @return {Integer}
def count_k_difference(nums, k)
  count = 0

  (0...nums.size - 1).each do |i|
    (i + 1...nums.size).each do |j|
      count += 1 if (nums[i] - nums[j]).abs == k
    end
  end

  count
end
