# @param {String} s
# @return {String}
def reverse_only_letters(s)
  alph = [*('a'..'z'), *('A'..'Z')]
  letters = s.chars.select {|c| alph.include?(c) }
  s.chars.map {|c| alph.include?(c) ? letters.pop : c }.join
end
