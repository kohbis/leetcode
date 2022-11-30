# @param {Integer[]} nums
# @return {Integer}
def sum_of_unique(nums)
  sum, count = 0, Hash.new(0)
  nums.each { |n| count[n] += 1 }
  count.each do |k, v|
    sum += k if v == 1
  end
  sum
end
