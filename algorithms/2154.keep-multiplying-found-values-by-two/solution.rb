# @param {Integer[]} nums
# @param {Integer} original
# @return {Integer}
def find_final_value(nums, original)
  target = original

  while true
    if nums.include?(target)
      target *= 2
    else
      break
    end
  end

  target
end
