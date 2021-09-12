# @param {String} s
# @return {Integer}
def first_uniq_char(s)
  ans = -1
  s.chars.uniq.each do |c|
    break ans = s.index(c) if s.count(c) == 1
  end
  ans
end
