# @param {Integer[][]} grid
# @return {Integer}
def max_increase_keeping_skyline(grid)
  row, col = grid.map(&:max), grid.transpose.map(&:max)

  res = 0
  (0...grid.size).map do |i|
    (0...grid[0].size).map do |j|
      res += [row[i], col[j]].min - grid[i][j]
    end
  end

  res
end
