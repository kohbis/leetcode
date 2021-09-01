# @param {String} s
# @return {Integer}
def length_of_longest_substring(s)
  curr = []
  longest = 0

  s.each_char do |c|
    if curr.include?(c)
      idx = curr.index(c)
      curr = curr[(idx + 1)..-1]
    end

    curr.push(c)
    longest = [longest, curr.size].max
  end

  longest
end
