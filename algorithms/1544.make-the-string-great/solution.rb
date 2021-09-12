# @param {String} s
# @return {String}
def make_good(s)
  stack = []
  s.chars.each do |c|
    if !stack.empty? && ((stack[-1].is_upper? && stack[-1].downcase == c) || (stack[-1].is_lower? && stack[-1].upcase == c))
      stack.pop
    else
      stack.push(c)
    end
  end
  stack.join
end

class String
  def is_upper?
    self >= "A" && "Z" >= self
  end

  def is_lower?
    self >= "a" && "z" >= self
  end
end
