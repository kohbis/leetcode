# @param {String} s
# @return {String[]}
def cells_in_range(s)
  row_start, row_end = s[0], s[3]
  col_start, col_end = s[1], s[4]

  list = []
  for row in row_start..row_end
    for col in col_start.to_i..col_end.to_i
      list << "#{row}#{col}"
    end
  end

  list
end
