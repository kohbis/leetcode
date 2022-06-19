# @param {String} column_title
# @return {Integer}
def title_to_number(column_title)
  res = 0

  column_title.chars.each do |c|
    n = c.ord - "A".ord + 1
    res = res * 26 + n
  end

  res
end
