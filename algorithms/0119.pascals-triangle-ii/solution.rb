# @param {Integer} row_index
# @return {Integer[]}
def get_row(row_index)
  rows = [[1]]

  # generate
  1.upto(row_index) do |i|
    # init row
    rows << [1] * (i + 1)

    # calculation
    (1...i).each do |j|
      rows[i][j] = rows[i - 1][j - 1] + rows[i - 1][j]
    end
  end

  rows[-1]
end
