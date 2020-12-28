# @param {String} pattern
# @param {String} s
# @return {Boolean}
def word_pattern(pattern, s)
  hash = {}
  words = s.split
  chars = pattern.chars

  return false unless words.size == chars.size

  chars.each_with_index do |c, i|
    if hash[c]
      return false unless hash[c] == words[i]
    else
      hash[c] = words[i]
    end
  end

  values = hash.values.sort
  values.size == values.uniq.size
end
