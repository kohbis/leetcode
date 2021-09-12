# @param {Integer[]} nums
# @param {Integer} target
# @return {Integer}
def search(nums, target)
  ## Using Array#bsearch_index
  # nums.bsearch_index {|x| target - x } || -1

  left, right = 0, nums.size - 1
  while left <= right
    mid = (left + right) / 2

    if target == nums[mid]
      return mid
    elsif target < nums[mid]
      right = mid - 1
    else
      left = mid + 1
    end
  end
  -1
end
