# @param {String} s
# @return {String}
def replace_digits(s)
  s.chars.each_slice(2).flat_map { |c, n|
    n ? [c.ord, c.ord + n.ord - '0'.ord] : c.ord
  }.pack('c*')

  # alph = ('a'..'z').to_a
  # s.chars.each_slice(2).map {|c, n|
  # n ? [c, alph[alph.find_index(c) + n.to_i]] : c
  # }.join
end
