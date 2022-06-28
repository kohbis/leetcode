# @param {String} word
# @return {Boolean}
def detect_capital_use(word)
  # ruby has useful methods!!!
  # word == word.capitalize || word == word.upcase || word == word.downcase

  uppers = word.chars.count { |c| c.is_upper? }
  uppers == 0 || uppers == word.length || (uppers == 1 && word[0].is_upper?)
end

class String
  def is_upper?
    self >= 'A' && 'Z' >= self
  end
end
