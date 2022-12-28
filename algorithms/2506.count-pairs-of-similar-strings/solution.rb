# @param {String[]} words
# @return {Integer}
def similar_pairs(words)
  words.map {|word| word.bytes.to_set }.tally.sum {|_, count| count * (count - 1) / 2 }
end
