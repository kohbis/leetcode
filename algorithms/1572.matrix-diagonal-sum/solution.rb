# @param {Integer[][]} mat
# @return {Integer}
def diagonal_sum(mat)
  sum = 0
  i = mat.size / 2

  if mat.size % 2 == 1
    sum += mat[i][i]
  end

  i -= 1

  while i >= 0
    sum += mat[i][i] + mat[i][-i-1] + mat[-i-1][i] + mat[-i-1][-i-1]
    i -= 1
  end

  sum
end
