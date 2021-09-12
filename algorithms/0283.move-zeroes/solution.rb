# @param {Integer[]} nums
# @return {Void} Do not return anything, modify nums in-place instead.
def move_zeroes(nums)
  cnt = 0
  nums.each_with_index do |n, idx|
    if n.zero?
      cnt += 1
    else
      nums[idx] = 0
      nums[idx - cnt] = n
    end
  end
end
