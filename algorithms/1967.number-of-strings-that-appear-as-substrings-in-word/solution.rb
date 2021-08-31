# @param {String[]} patterns
# @param {String} word
# @return {Integer}
def num_of_strings(patterns, word)
  patterns.count {|p| word.include?(p) }
end
