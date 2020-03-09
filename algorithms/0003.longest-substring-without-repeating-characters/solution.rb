# @param {String} s
# @return {Integer}
def length_of_longest_substring(s)
  return s.size if s.size < 2

  longest, curt = 0, []

  s.each_char do |c|
    if curt.include?(c)
      longest = [longest, curt.size].max
      idx = curt.index(c)
      curt = curt[idx+1..-1]
    end
    curt << c
  end

  [longest, curt.size].max
end
