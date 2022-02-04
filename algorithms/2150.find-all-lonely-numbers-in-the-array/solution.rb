# @param {Integer[]} nums
# @return {Integer[]}
def find_lonely(nums)
  h = Hash.new(0)
  nums.each { |n| h[n] += 1 }

  ans = []
  h.each do |k, v|
    if v == 1 && h[k + 1] == 0 && h[k - 1] == 0
      ans << k
    end
  end

  ans
end
