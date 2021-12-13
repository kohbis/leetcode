# @param {String[]} words1
# @param {String[]} words2
# @return {Integer}
def count_words(words1, words2)
  (words1.tally.select { |_, v| v == 1 }.keys & words2.tally.select { |_, v| v == 1 }.keys).size
end
