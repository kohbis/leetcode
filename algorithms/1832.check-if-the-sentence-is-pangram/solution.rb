# @param {String} sentence
# @return {Boolean}
def check_if_pangram(sentence)
  h = ('a'..'z').to_a.map {|alph| [alph, 0] }.to_h
  sentence.each_char do |c|
    h[c] += 1
  end
  h.key(0).nil?
end
