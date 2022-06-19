# @param {String} s
# @return {Integer}
def first_uniq_char(s)
  res = -1
  s.chars.uniq.each do |c|
    break res = s.index(c) if s.count(c) == 1
  end
  res
end
