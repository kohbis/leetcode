# @param {Integer[]} nums
# @param {Integer} k
# @return {Integer}
def find_pairs(nums, k)
  ans = 0

  hash = Hash.new(0)
  nums.each { |n| hash[n] += 1 }

  hash.each do |key, value|
    if hash.has_key?(key + k)
      if (k == 0 && value > 1) || k > 0
        ans += 1
      end
    end
  end

  ans
end
