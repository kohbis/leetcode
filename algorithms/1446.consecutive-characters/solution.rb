# @param {String} s
# @return {Integer}
def max_power(s)
  max_count = 0
  previous_character = ""

  count = 0
  s.chars.each do |c|
    if previous_character == c
      count += 1
    else
      count = 1
      previous_character = c
    end

    max_count = [count, max_count].max
  end

  max_count
end
