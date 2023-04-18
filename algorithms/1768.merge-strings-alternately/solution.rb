# @param {String} word1
# @param {String} word2
# @return {String}
def merge_alternately(word1, word2)
  res = ""
  i = 0
  while i < word1.size || i < word2.size
    res << word1[i] if i < word1.size
    res << word2[i] if i < word2.size
    i += 1
  end
  res
end
