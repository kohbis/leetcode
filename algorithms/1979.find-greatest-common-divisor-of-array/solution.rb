# @param {Integer[]} nums
# @return {Integer}
def find_gcd(nums)
  min, max = 1001, 0

  nums.each do |n|
    min = n if n < min
    max = n if n > max
  end

  gcd(max, min)
end

def gcd(x, y)
  return x if y == 0
  gcd(y, x % y)
end
