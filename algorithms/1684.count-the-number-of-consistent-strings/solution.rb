# @param {String} allowed
# @param {String[]} words
# @return {Integer}
def count_consistent_strings(allowed, words)
  count = 0
  allowed_chars = allowed.chars

  words.each do |word|
    only_allowed_chars = true
    word_chars = word.each_char.sort.uniq

    word_chars.each do |c|
      unless allowed_chars.include?(c)
        only_allowed_chars = false
        break
      end
    end
    count += 1 if only_allowed_chars
  end

  count
end
