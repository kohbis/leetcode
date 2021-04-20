# @param {String} sentence
# @return {Boolean}
def check_if_pangram(sentence)
  # easy 1-liner solution
  # sentence.chars.sort.uniq.size == 26

  # using Hash
  h = Hash.new(0)
  sentence.each_char { |c| h[c] += 1 }
  h.keys.size == 26
end
