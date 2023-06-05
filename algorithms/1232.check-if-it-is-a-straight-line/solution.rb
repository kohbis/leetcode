# @param {Integer[][]} coordinates
# @return {Boolean}
def check_straight_line(coordinates)
  return true if coordinates.length <= 2

  x1 = coordinates[0][0]
  y1 = coordinates[0][1]
  x2 = coordinates[1][0]
  y2 = coordinates[1][1]

  2.upto(coordinates.length - 1) do |i|
    x = coordinates[i][0]
    y = coordinates[i][1]

    # x1-x:y1-y = x2-x1:y2-y1
    return false if (x1 - x) * (y2 - y1) != (y1 - y) * (x2 - x1)
  end

  true
end
