# @param {String} text
# @return {Integer}
def max_number_of_balloons(text)
  arr = text.chars
  [arr.count('b'), arr.count('a'), (arr.count('l') / 2), (arr.count('o') / 2), arr.count('n')].min
end
