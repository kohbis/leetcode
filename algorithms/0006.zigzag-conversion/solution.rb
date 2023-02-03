# @param {String} s
# @param {Integer} num_rows
# @return {String}
def convert(s, num_rows)
  return s if num_rows == 1 || s.length <= num_rows

  rows = Array.new(num_rows) { Array.new }
  row = 0
  i = 1

  s.each_char do |c|
    rows[row] << c

    if row == 0
      i = 1
    elsif row == num_rows - 1
      i = -1
    end

    row += i
  end

  rows.inject('') { |res, row| res + row.join }
end
