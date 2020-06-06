# @param {Integer[]} nums
# @return {Integer}
def max_product(nums)
  # nums.max(2).inject(1) { |res, num| res * (num - 1) }
  
  max, second_max = 0, 0
  nums.each do |num|
    if num > max then
      second_max = max
      max = num
    elsif num > second_max then
      second_max = num
    end
  end
  (max - 1) * (second_max - 1)
end
