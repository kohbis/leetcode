# @param {Integer[]} original
# @param {Integer} m
# @param {Integer} n
# @return {Integer[][]}
def construct2_d_array(original, m, n)
  if original.size == m * n
    original.each_slice(n).to_a
  else
    []
  end
end
