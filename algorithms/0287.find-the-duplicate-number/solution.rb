# @param {Integer[]} nums
# @return {Integer}
def find_duplicate(nums)
  set = Set.new
  nums.each do |num|
    if set.include?(num)
      return num
    else
      set << num
    end
  end
end
