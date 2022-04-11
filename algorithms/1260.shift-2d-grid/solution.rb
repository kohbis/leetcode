# @param {Integer[][]} grid
# @param {Integer} k
# @return {Integer[][]}
def shift_grid(grid, k)
  horiz = grid.flatten
  (k % horiz.size).times do
    tmp = horiz.pop
    horiz.insert(0, tmp)
  end

  horiz.each_slice(grid[0].size).to_a
end
