# Runtime: 80ms, Memory: 209.7MB
# @param {Integer[]} nums
# @return {Integer}
def special_array(nums)
  hash = {}
  nums.each do |n|
    hash[n] = 0 unless hash.key?(n)
    hash[n] += 1
  end

  (1..nums.size).each do |x|
    return x if x == hash.select { |k, _| k >= x }.values.sum
  end

  -1
end

# Runtime: 136ms, Memory: 216.8MB
# #@param {Integer[]} nums
# # @return {Integer}
# def special_array(nums)
#   nums.size.times do |i|
#     x = i + 1
#     return x if nums.count { |n| n >= x } == x
#   end

#   -1
# end
