# @param {String} s
# @return {Boolean}
def check_string(s)
  appeared = false

  s.chars.each do |c|
    case c
    when 'a'
      return false if appeared
    when 'b'
      appeared = true
    end
  end

  true
end
