# @param {String[]} words
# @return {Integer}
def minimum_length_encoding(words)
  words.sort_by! { |word| word.length }.reverse!
  str = ""
  words.each do |word|
    str << "#{word}#" unless str.include?("#{word}#")
  end
  str.length
end
