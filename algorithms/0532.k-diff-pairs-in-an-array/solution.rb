# @param {Integer[]} nums
# @param {Integer} k
# @return {Integer}
def find_pairs(nums, k)
  ans = 0

  hash = Hash.new(0)
  nums.each { |n| hash[n] += 1 }

  if k == 0
    ans = hash.count { |_, v| v > 1 }
  else
    hash.keys.each do |n|
      ans += 1 if hash[n + k] > 0
    end
  end

  ans
end
