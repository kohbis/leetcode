# @param {Integer[][]} matrix
# @return {Integer[]}
def lucky_numbers (matrix)
  matrix.transpose.map {|row| row.max } & matrix.map {|col| col.min }
end
