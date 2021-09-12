# @param {Integer[]} arr
# @return {Boolean}
def can_make_arithmetic_progression(arr)
  ## one liner ##
  # arr.sort.each_cons(2).map {|a, b| a - b }.uniq.size == 1

  ## calc each combinations ##
  arr.sort!
  diff = arr[0] - arr[1]
  (1...arr.size - 1).each do |i|
    return false unless diff == arr[i] - arr[i + 1]
  end
  # default
  true
end
