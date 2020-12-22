 # @param {String} s
# @return {Integer}
def max_depth(s)
  max, current = 0, 0

  s.each_char do |c|
    if c == "("
      current += 1
      max = [max, current].max
    elsif c == ")"
      current -= 1 if current > 0
    end
  end

  max
end
