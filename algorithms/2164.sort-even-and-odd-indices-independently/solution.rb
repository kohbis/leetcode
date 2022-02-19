# @param {Integer[]} nums
# @return {Integer[]}
def sort_even_odd(nums)
  odds, evens = [], []
  (0...nums.size).each do |i|
    if i % 2 == 0
      evens << nums[i]
    else
      odds << nums[i]
    end
  end

  odds.sort!
  evens.sort!

  evens.zip(odds.reverse).flatten.compact
end
