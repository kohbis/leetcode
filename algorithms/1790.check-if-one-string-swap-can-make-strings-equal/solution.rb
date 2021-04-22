# @param {String} s1
# @param {String} s2
# @return {Boolean}
def are_almost_equal(s1, s2)
  a, b = [], []
  (0..s1.size).each do |i|
    unless s1[i] == s2[i]
      a << s1[i]
      b << s2[i]
      return false if a.size > 2
    end
  end
  a.sort == b.sort
end
