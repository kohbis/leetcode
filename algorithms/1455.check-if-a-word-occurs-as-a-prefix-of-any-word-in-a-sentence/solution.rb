# @param {String} sentence
# @param {String} search_word
# @return {Integer}
def is_prefix_of_word(sentence, search_word)
  sentence.split.each_with_index { |word, i| return i + 1 if word.start_with?(search_word) }
  -1
end
