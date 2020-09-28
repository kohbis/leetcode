# @param {Integer[]} nums
# @param {Integer} k
# @return {Void} Do not return anything, modify nums in-place instead.
def rotate(nums, k)
  k.times do
    # nums.rotate!(-1)

    nums.unshift(nums.pop)
  end
end
