# @param {Integer[]} nums
# @return {Boolean}
def divide_array(nums)
  set = Set.new

  nums.each do |n|
    if set.include?(n)
      set.delete(n)
    else
      set.add(n)
    end
  end

  set.size == 0
end
