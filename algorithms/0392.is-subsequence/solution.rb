# @param {String} s
# @param {String} t
# @return {Boolean}
def is_subsequence(s, t)
  letters = t.chars

  s.chars do |c|
    idx = letters.index(c)
    if idx
      letters = letters[idx + 1..]
    else
      return false
    end
  end

  true
end
