Alph = *("a".."z")

# @param {String} s
# @return {String}
def freq_alphabets(s)
  res, chars = [], s.chars
  until chars.size == 0
    c = chars.pop
    idx = (c == "#" ? chars.pop(2).join.to_i : c.to_i) - 1
    res.unshift Alph[idx]
  end
  res.join
end
