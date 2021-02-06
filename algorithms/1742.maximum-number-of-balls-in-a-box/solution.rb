# @param {Integer} low_limit
# @param {Integer} high_limit
# @return {Integer}
def count_balls(low_limit, high_limit)
  ## One liner solution
  # (low_limit..high_limit).map { |n| n.digits.sum }.tally.values.max

  hash = Hash.new(0)
  (low_limit..high_limit).each do |n|
    key = n.digits.sum
    hash[key] += 1
  end
  hash.values.max
end
