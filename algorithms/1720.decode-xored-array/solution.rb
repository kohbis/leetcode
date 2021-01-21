# @param {Integer[]} encoded
# @param {Integer} first
# @return {Integer[]}
def decode(encoded, first)
  encoded.unshift(first)

  for i in 0...encoded.size-1
    encoded[i+1] = encoded[i] ^ encoded[i+1]
  end

  encoded
end
