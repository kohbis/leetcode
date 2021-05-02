# @param {String} s
# @return {String}
def replace_digits(s)
  alph = ('a'..'z').to_a
  s.chars.each_slice(2).map {|c, n|
    n ? [c, alph[alph.find_index(c) + n.to_i]] : c
  }.join
end
