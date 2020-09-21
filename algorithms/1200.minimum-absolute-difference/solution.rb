# @param {Integer[]} arr
# @return {Integer[][]}
def minimum_abs_difference(arr)
  res = []
  min_diff = -1
  arr.sort.each_cons(2) do |a, b|
    diff = b - a
    min_diff = diff unless min_diff > 0

    if min_diff > diff
      min_diff = diff
      res.clear.push [a, b]
    elsif min_diff == diff
      res.push [a, b]
    end
  end

  res
end
