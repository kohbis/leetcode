# @param {String} s
# @return {Integer}
def balanced_string_split(s)
  res, cnt = 0, 0
  s.chars.each do |c|
    if c == 'L'
      cnt += 1
    else
      cnt -= 1
    end
    res += 1 if cnt == 0
  end
  res
end
