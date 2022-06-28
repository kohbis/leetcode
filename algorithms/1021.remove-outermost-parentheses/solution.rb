# @param {String} s
# @return {String}
def remove_outer_parentheses(s)
  res = ''
  open_cnt = 0
  s.each_char do |c|
    res << c unless (open_cnt == 0 && c == '(') || (open_cnt == 1 && c == ')')
    open_cnt += c == '(' ? 1 : -1
  end
  res
end
