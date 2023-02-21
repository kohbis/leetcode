# @param {Integer[]} nums
# @return {Integer}
def single_non_duplicate(nums)
  set = Set.new
  nums.each do |n|
    set.delete(n) unless set.add?(n)
  end
  set.first
end
