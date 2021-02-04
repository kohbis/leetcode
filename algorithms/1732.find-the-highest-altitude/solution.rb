# @param {Integer[]} gain
# @return {Integer}
def largest_altitude(gain)
  max = 0
  pos = 0

  gain.each do |g|
    pos += g
    max = pos if pos > max
  end

  max
end
