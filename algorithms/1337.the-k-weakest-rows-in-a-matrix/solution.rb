# @param {Integer[][]} mat
# @param {Integer} k
# @return {Integer[]}
def k_weakest_rows(mat, k)
  mat.each_with_index.map { |soldiers, i| [i, soldiers.sum] }.sort_by { |v| v[1] }.first(k).map { |v| v[0] }
end
