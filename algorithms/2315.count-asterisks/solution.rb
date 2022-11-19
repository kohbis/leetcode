# @param {String} s
# @return {Integer}
def count_asterisks(s)
  count = 0
  inside = true

  s.chars.each do |c|
    case c
    when '|'
      inside = !inside
    when '*'
      count += 1 if inside
    end
  end

  count
end
