# @param {String} digits
# @return {String[]}
def letter_combinations(digits)
  return [] if digits.empty?

  buttons = [
    [],
    [], ['a', 'b', 'c'], ['d', 'e', 'f'],
    ['g', 'h', 'i'], ['j', 'k', 'l'], ['m', 'n', 'o'],
    ['p', 'q', 'r', 's'], ['t', 'u', 'v'], ['w', 'x', 'y', 'z'],
  ]
  letters = digits.chars.map { |d| buttons[d.to_i] }
  [''].product(*letters).map(&:join)
end
