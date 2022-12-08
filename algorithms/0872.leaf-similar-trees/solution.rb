# Definition for a binary tree node.
# class TreeNode
#     attr_accessor :val, :left, :right
#     def initialize(val = 0, left = nil, right = nil)
#         @val = val
#         @left = left
#         @right = right
#     end
# end
# @param {TreeNode} root1
# @param {TreeNode} root2
# @return {Boolean}
def leaf_similar(root1, root2)
  seq_left = leaf_sequence(root1)
  seq_right = leaf_sequence(root2)
  seq_left == seq_right
end

def leaf_sequence(root, current = [])
  return current if root.nil?
  current << root.val if root.left.nil? && root.right.nil?

  leaf_sequence(root.left, current)
  leaf_sequence(root.right, current)
end
