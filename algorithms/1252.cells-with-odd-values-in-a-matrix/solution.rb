def odd_cells(n, m, indices)
  matrix = Array.new(n) { Array.new(m, 0) }
  count = 0

  # apply the increment to all indices
  indices.each do |ind|
    ri, ci = ind[0], ind[1]
    # row
    matrix[ri].map! { |cell| cell = cell + 1 }
    # column
    matrix.each { |row| row[ci] = row[ci] + 1 }
  end

  # matrix.flatten.count {|cell| cell % 2 != 0 }
  matrix.each do |row|
    count += row.count { |cell| cell % 2 != 0 }
  end

  count
end
