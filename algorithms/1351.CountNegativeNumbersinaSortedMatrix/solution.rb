# @param {Integer[][]} grid
# @return {Integer}
def count_negatives(grid)
  grid.inject(0) {|res, item| res + item.select {|n| n.negative? }.count }
end
