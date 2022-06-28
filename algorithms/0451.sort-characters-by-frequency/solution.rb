# @param {String} s
# @return {String}
def frequency_sort(s)
  counter = Hash.new(0)
  s.each_char { |c| counter[c] += 1 }
  counter.sort_by { |_, v| -v }.inject('') { |res, item| res + item[0] * item[1] }
end
