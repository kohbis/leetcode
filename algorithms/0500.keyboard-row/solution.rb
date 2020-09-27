# @param {String[]} words
# @return {String[]}
def find_words(words)
  rows = ["qwertyuiop", "asdfghjkl", "zxcvbnm"]
  words.select do |word|
    word_chars = word.downcase.chars

    word_chars.all? {|c| rows[0].include?(c) } ||
      word_chars.all? {|c| rows[1].include?(c) } ||
      word_chars.all? {|c| rows[2].include?(c) }
  end
end
