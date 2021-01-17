VOWELS = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']

# @param {String} s
# @return {Boolean}
def halves_are_alike(s)
  a_vowels, b_vowels = 0, 0

  mid = s.size / 2
  (0...mid).each do |i|
    a_vowels += 1 if VOWELS.include?(s[i])
    b_vowels += 1 if VOWELS.include?(s[-i-1])
  end

  a_vowels == b_vowels
end
