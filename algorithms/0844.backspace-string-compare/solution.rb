def string_builder(str)
  arr = []
  str.chars.each do |char|
    if "#" == char
      arr.pop
    else
      arr.push(char)
    end
  end
  arr.join
end

# @param {String} s
# @param {String} t
# @return {Boolean}
def backspace_compare(s, t)
  string_builder(s) == string_builder(t)
end
