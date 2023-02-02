# @param {String[]} words
# @param {String} order
# @return {Boolean}
def is_alien_sorted(words, order)
  traslator = order.chars.map.with_index { |c, i| [c, (i + 97).chr] }.to_h
  translated = words.map { |word| translate(traslator, word) }
  translated == translated.sort
end

def translate(translator, word)
  word.chars.map { |c| translator[c] }.join
end
