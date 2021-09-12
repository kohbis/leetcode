# @param {Integer[][]} coordinates
# @return {Boolean}
def check_straight_line(coordinates)
  is_straight = true

  return is_straight if coordinates.length <= 2

  x1, y1 = coordinates[0][0], coordinates[0][1]
  x2, y2 = coordinates[1][0], coordinates[1][1]

  2.upto(coordinates.length - 1) do |i|
    x, y = coordinates[i][0], coordinates[i][1]
    # x1-x:y1-y = x2-x1:y2-y1
    is_straight = false if (x1 - x) * (y2 - y1) != (y1 - y) * (x2 - x1)
  end

  is_straight
end
