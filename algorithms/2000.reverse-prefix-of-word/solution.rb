# @param {String} word
# @param {Character} ch
# @return {String}
def reverse_prefix(word, ch)
  idx = word.index(ch)
  if idx
    word[0..idx].reverse + word[idx + 1..]
  else
    word
  end
end
