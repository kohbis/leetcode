# @param {Integer[]} encoded
# @param {Integer} first
# @return {Integer[]}
def decode(encoded, first)
  res = [first]

  for i in 0...encoded.size
    res << (res[i] ^ encoded[i])
  end

  res
end
