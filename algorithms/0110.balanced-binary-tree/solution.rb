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
# @return {Boolean}
def is_balanced(root)
  @balanced = true
  dfs(root, 0)
  @balanced
end

def dfs(node, height)
  return height - 1 if node.nil?

  left = dfs(node.left, height + 1)
  right = dfs(node.right, height + 1)

  @balanced = false if (left - right).abs > 1

  [left, right].max
end
