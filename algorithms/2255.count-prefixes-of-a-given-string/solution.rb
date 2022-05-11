# @param {String[]} words
# @param {String} s
# @return {Integer}
def count_prefixes(words, s)
  words.count { |word| s.start_with?(word) }
end
