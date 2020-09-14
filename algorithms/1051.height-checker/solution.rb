# @param {Integer[]} heights
# @return {Integer}
def height_checker(heights)
  target = heights.sort
  (0...heights.size).count {|i| heights[i] != target[i] }
end
