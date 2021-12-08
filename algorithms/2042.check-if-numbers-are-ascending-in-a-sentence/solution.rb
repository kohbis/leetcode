# @param {String} s
# @return {Boolean}
def are_numbers_ascending(s)
  prev = 0
  s.split.each do |x|
    if x.to_i.to_s == x
      if prev < x.to_i
        prev = x.to_i
      else
        return false
      end
    end
  end

  true
end
