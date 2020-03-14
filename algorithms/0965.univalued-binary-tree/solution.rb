# Definition for a binary tree node.
# class TreeNode
#     attr_accessor :val, :left, :right
#     def initialize(val)
#         @val = val
#         @left, @right = nil, nil
#     end
# end

# @param {TreeNode} root
# @return {Boolean}
def is_unival_tree(root)
  return true if root.nil?
  l = root.left.nil? || (root.val == root.left.val)
  r = root.right.nil? || (root.val == root.right.val)
  l && r && is_unival_tree(root.left) && is_unival_tree(root.right)
end
