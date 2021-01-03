# @param {String} a
# @param {String} b
# @return {String[]}
def uncommon_from_sentences(a, b)
  hash = {} # {word => count}
  [a, b].each do |sentence|
    sentence.split.each do |word|
      hash[word] = 0 unless hash.has_key?(word)
      hash[word] += 1
    end
  end

  hash.select {|_, v| v == 1}.keys
end

