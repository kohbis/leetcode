# @param {Integer[][]} mat
# @return {Integer[][]}
def diagonal_sort(mat)
  diag_hash = Hash.new { |h, k| h[k] = [] }

  for i in 0...mat.size
    for j in 0...mat[0].size
      diag_hash[i - j] << mat[i][j]
    end
  end

  diag_hash.each { |_, v| v.sort! }

  for i in 0...mat.size
    for j in 0...mat[0].size
      mat[i][j] = diag_hash[i - j].shift
    end
  end

  mat
end
