# @param {String[]} words
# @return {String[]}
def string_matching(words)
  res = []
  words = words.sort_by(&:length)

  (0...words.size - 1).each do |i|
    (i + 1...words.size).each do |j|
      if words[j][words[i]]
        res << words[i]
        break
      end
    end
  end

  res
end
