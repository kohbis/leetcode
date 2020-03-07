# @param {Integer[]} nums
# @return {Integer}
def find_unsorted_subarray(nums)
  sorted = nums.sort
  l = nums.length
  r = 0
  nums.each_with_index do |n, i|
    if n != sorted[i]
      l = [l, i].min
      r = [r, i].max
    end
  end
  (r - l).positive? ? (r - l + 1) : 0
end
