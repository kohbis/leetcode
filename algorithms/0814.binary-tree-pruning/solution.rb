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
def prune_tree(root)
  return root if root.nil?

  root.left = prune_tree(root.left) if root.left
  root.right = prune_tree(root.right) if root.right

  if root.val == 0 && root.left.nil? && root.right.nil?
    return nil
  end

  root
end
