# @param {String} s
# @param {String} t
# @return {Character}
def find_the_difference(s, t)
  letters = t.chars
  s.each_char { |c| letters.delete_at(letters.find_index(c)) }
  letters[0]
end
