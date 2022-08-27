# @param {Integer[][]} grid
# @return {Integer[][]}
def largest_local(grid)
  Array.new(grid.size - 2) do |i|
    Array.new(grid[i].size - 2) do |j|
      grid[i..i + 2].flat_map { |g| g[j..j + 2] }.max
    end
  end
end
