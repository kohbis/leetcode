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
# @param {Integer} low
# @param {Integer} high
# @return {TreeNode}
def trim_bst(root, low, high)
  return nil if root.nil?

  return trim_bst(root.left, low, high) if high < root.val
  return trim_bst(root.right, low, high) if root.val < low

  root.left = trim_bst(root.left, low, high) if root.left
  root.right = trim_bst(root.right, low, high) if root.right

  root
end
