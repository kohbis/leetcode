# @param {Integer[]} nums
# @return {Integer}
def sum_of_unique(nums)
  sum, counts = 0, Hash.new(0)
  nums.each { |n| counts[n] += 1 }
  counts.each do |k, v|
    sum += k if v == 1
  end
  sum
end
