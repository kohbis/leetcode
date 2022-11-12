# @param {Integer[]} nums
# @param {Integer} pivot
# @return {Integer[]}
def pivot_array(nums, pivot)
  lesser, equal, greater = [], [], []
  nums.each do |n|
    if n == pivot
      equal << n
    elsif n < pivot
      lesser << n
    else
      greater << n
    end
  end
  lesser + equal + greater
end
