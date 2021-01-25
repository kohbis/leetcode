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
# @return {Integer}
def deepest_leaves_sum(root)
  deepest_recursion(root, 0, Hash.new(0)).max[1]
end

def deepest_recursion(node, level = 0, hash = {})
  deepest_recursion(node.left, level + 1, hash) if node.left
  deepest_recursion(node.right, level + 1, hash) if node.right

  hash[level] += node.val

  hash
end
