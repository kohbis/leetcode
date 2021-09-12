# @param {String} j
# @param {String} s
# @return {Integer}
def num_jewels_in_stones(j, s)

  # cnt = 0
  # j.chars.map { |c| cnt += s.count(c) }
  # cnt

  j.chars.inject(0) { |ans, c| ans += s.count(c) }
end
