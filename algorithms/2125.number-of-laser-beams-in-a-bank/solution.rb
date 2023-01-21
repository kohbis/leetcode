# @param {String[]} bank
# @return {Integer}
def number_of_beams(bank)
  rows = bank.map { |floor| floor.count('1') }.reject { |cnt| cnt == 0 }
  count = 0
  for i in 0...rows.size - 1
    count += rows[i] * rows[i + 1]
  end
  count
end
