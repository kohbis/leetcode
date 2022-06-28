# @param {String} word1
# @param {String} word2
# @return {String}
def merge_alternately(word1, word2)
  chars1 = word1.chars
  chars2 = word2.chars

  res = ''
  while !chars1.empty? || !chars2.empty?
    res += chars1.shift unless chars1.empty?
    res += chars2.shift unless chars2.empty?
  end

  res
end
