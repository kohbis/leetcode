# @param {String} s
# @return {String}
def greatest_letter(s)
  res = ''
  set = Set.new

  s.chars.each do |c|
    uc = c.upcase
    if (c == uc && set.include?(c.downcase)) || (c != uc && set.include?(uc))
      res = uc if res < uc
    end

    set << c
  end

  res
end
