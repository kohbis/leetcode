# @param {String} s
# @return {Integer[]}
def di_string_match(s)
  nums = *(0..s.length)
  res = []
  s.each_char do |c|
    res.push c == 'I' ? nums.shift : nums.pop
  end
  res.push nums.pop
  res
end

