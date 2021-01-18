# @param {Integer} n
# @return {String}
def count_and_say(n)
  return "1" if n == 1

  count_and_say(n-1)
    .chars
    .chunk_while {|a, b| a == b }
    .map {|chunk| chunk.size.to_s + chunk[0].to_s }
    .join
end
