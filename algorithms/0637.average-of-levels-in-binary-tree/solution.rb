# Definition for a binary tree node.
# class TreeNode
#     attr_accessor :val, :left, :right
#     def initialize(val = 0, left = nil, right = nil)
#         @val = val
#         @left = left
#         @right = right
#     end
# end
# @param {TreeNode} root
# @return {Float[]}
def average_of_levels(root)
  res = []
  nodes = []

  nodes << root if root

  while !nodes.empty?
    len = nodes.size
    sum = 0.0

    len.times do
      node = nodes.shift
      next unless node

      sum += node.val

      nodes << node.left if node.left
      nodes << node.right if node.right
    end

    res << sum / len
  end

  res
end
