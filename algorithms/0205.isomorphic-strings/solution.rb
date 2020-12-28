# @param {String} s
# @param {String} t
# @return {Boolean}
def is_isomorphic(s, t)
  return false unless s.length == t.length

  hash = {}
  t_chars = t.chars

  s.each_char.with_index do |c, i|
    if hash[c]
      return false unless hash[c] == t_chars[i]
    else
      hash[c] = t_chars[i]
    end
  end

  values = hash.values.sort
  values.size == values.uniq.size
end

