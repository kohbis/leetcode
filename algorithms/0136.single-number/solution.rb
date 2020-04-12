# @param {Integer[]} nums
# @return {Integer}
def single_number(nums)
  ## XOR ##
  nums.reduce(&:^)

  ## EACH ##
  # temp = []
  # nums.each do |n|
  #   if temp.include?(n)
  #     temp.delete(n)
  #   else
  #     temp << n
  #   end
  # end
  # temp[0]
end
