# @param {String} s
# @return {Boolean}
def is_valid(s)
  stack = []
  brackets = { '(' => ')', '{' => '}', '[' => ']' }

  s.each_char do |c|
    if brackets.has_key?(c)
      stack.push(c)
    elsif brackets.has_value?(c)
      return false if brackets[stack.pop] != c
    else
      return false
    end
  end

  stack.empty?
end
