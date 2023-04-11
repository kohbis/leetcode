# @param {String} s
# @return {String}
def remove_stars(s)
  stack = []
  s.chars.each do |c|
    if c == '*'
      stack.pop
    else
      stack << c
    end
  end
  stack.join
end
