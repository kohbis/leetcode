# @param {String} s
# @param {Integer[][]} shift
# @return {String}
def string_shift(s, shift)
  part = shift.inject(0) do |total, val|
    # dirction 0:left 1:right
    total += val[0].zero? ? val[1] : -val[1]
  end % s.length

  part.zero? ? s : s[part..-1] + s[0..part-1]
end

