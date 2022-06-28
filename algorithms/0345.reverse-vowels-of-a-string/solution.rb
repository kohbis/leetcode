# @param {String} s
# @return {String}
def reverse_vowels(s)
  vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']
  stack = s.chars.select { |x| vowels.include?(x) }

  res = []
  s.chars.each do |c|
    if vowels.include?(c)
      res << stack.pop
    else
      res << c
    end
  end

  res.join
end
