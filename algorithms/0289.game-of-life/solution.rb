# @param {Integer[][]} board
# @return {Void} Do not return anything, modify board in-place instead.
def game_of_life(board)
  row_size, col_size = board.size, board[0].size
  tmp_board = Array.new(row_size) { Array.new(col_size, 0) }

  neighbors = [[1, 1], [1, 0], [1, -1], [0, -1], [-1, -1], [-1, 0], [-1, 1], [0, 1]]
  for row in 0...row_size
    for col in 0...col_size
      live_neighbors_count = 0
      for r, c in neighbors
        live_neighbors_count += 1 if neighbor_live?(board, row + r, col + c)
      end

      tmp_board[row][col] =
 # 1. Any live cell with fewer than two live neighbors dies as if caused by under-population.
         if board[row][col] == 1 && live_neighbors_count < 2
          0
          # 2. Any live cell with two or three live neighbors lives on to the next generation.
        elsif board[row][col] == 1 && (live_neighbors_count == 2 || live_neighbors_count == 3)
          1
          # 3. Any live cell with more than three live neighbors dies, as if by over-population.
        elsif board[row][col] == 1 && live_neighbors_count > 3
          0
          # 4. Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
        elsif board[row][col] == 0 && live_neighbors_count == 3
          1
        else
          board[row][col]
        end
    end
  end

  # replace
  for i in 0...row_size
    board[i] = tmp_board[i]
  end
end

# @param {Integer[][]} board
# @param {Integer} row
# @param {Integer} col
# @return {Boolean} alive
def neighbor_live?(board, row, col)
  row_size, col_size = board.size, board[0].size
  0 <= row && 0 <= col && row < board.size && col < board[0].size && board[row][col] == 1
end
