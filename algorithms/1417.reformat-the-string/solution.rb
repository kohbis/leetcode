# @param {String} s
# @return {String}
def reformat(s)
  alpha = []
  nums = []
  res = ''

  s.chars.each do |c|
    if /[[:alpha:]]/.match(c)
      alpha << c
    else
      nums << c
    end
  end

  return '' if (alpha.size - nums.size).abs > 1

  if alpha.size > nums.size
    while alpha.any?
      res << alpha.pop
      res << nums.pop unless nums.empty?
    end
  else
    while nums.any?
      res << nums.pop
      res << alpha.pop unless alpha.empty?
    end
  end

  res
end
