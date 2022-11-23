# @param {Character[][]} board
# @return {Boolean}
def is_valid_sudoku(board)
  rows = Array.new(9) { Set.new }
  cols = Array.new(9) { Set.new }
  boxes = Array.new(3) { Array.new(3) { Set.new } }

  board.each_with_index do |row, row_idx|
    row.each_with_index do |x, col_idx|
      next if x == '.'

      if rows[row_idx].include?(x)
        return false
      else
        rows[row_idx] << x
      end

      if cols[col_idx].include?(x)
        return false
      else
        cols[col_idx] << x
      end

      if boxes[row_idx / 3][col_idx / 3].include?(x)
        return false
      else
        boxes[row_idx / 3][col_idx / 3] << x
      end
    end
  end

  true
end
