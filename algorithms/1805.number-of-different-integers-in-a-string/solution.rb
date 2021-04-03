# @param {String} word
# @return {Integer}
def num_different_integers(word)
  word.scan(/\d+/).map(&:to_i).sort.uniq.size
end
