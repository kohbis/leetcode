# @param {Integer} n
# @param {Integer[][]} edges
# @return {Integer[]}
def find_smallest_set_of_vertices(n, edges)
  nodes = (0...n).to_a
  edges.each { nodes[_1[1]] = nil }
  nodes.compact
end
