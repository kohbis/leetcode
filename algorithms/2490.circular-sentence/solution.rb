# @param {String} sentence
# @return {Boolean}
def is_circular_sentence(sentence)
  words = sentence.split
  words << words[0]
  words.each_cons(2).all? { |a, b| a[-1] == b[0] }
end
