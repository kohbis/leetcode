# @param {Integer[][]} grid
# @return {Integer}
def delete_greatest_value(grid)
  grid.map(&:sort).transpose.sum(&:max)
end
