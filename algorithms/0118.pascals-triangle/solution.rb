# @param {Integer} num_rows
# @return {Integer[][]}
def generate(num_rows)
  res = []

  # generate
  num_rows.times do |i|
    # init row
    res << [1] * (i + 1)

    # calculation
    (1...i).each do |j|
      res[i][j] = res[i - 1][j - 1] + res[i - 1][j]
    end
  end

  res
end
