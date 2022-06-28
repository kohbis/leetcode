# @param {String} s
# @param {Integer} k
# @return {Boolean}
def has_all_codes(s, k)
  return false if s.size < k

  set = Set.new
  (0..(s.size - k)).each do |i|
    set << s[i...i + k]
  end

  set.size == 2 ** k
end
