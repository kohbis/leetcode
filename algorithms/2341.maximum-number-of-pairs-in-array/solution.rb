# @param {Integer[]} nums
# @return {Integer[]}
def number_of_pairs(nums)
  nums.tally.values.each_with_object([0, 0]) do |item, obj|
    obj[0] += item / 2
    obj[1] += item % 2
  end
end
