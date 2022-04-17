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
# @return {TreeNode}
def bst_to_gst(root)
  @sum = 0
  helper(root)

  root
end

def helper(node)
  return if node.nil?

  helper(node.right)
  node.val += @sum
  @sum = node.val
  helper(node.left)
end
